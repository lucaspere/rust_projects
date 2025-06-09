use std::{sync::Arc, time::Duration};

use futures::future;
use redis::AsyncCommands;
use snafu::ResultExt;
use tracing::{error, info, warn};

use crate::{
    error::RedisSnafu,
    model::dexevents,
    providers::StreamProvider,
    traits::{RealtimePublisher, RealtimePublisherExt, RealtimeSubscriber},
    DexMessagingResult,
};

const LOCK_TTL_SECS: u64 = 30;
const HEARTBEAT_INTERVAL_SECS: u64 = 15;

fn ref_count_key(stream_id: &str) -> String {
    format!("refcount:stream:{}", stream_id)
}
fn lock_key(stream_id: &str) -> String {
    format!("lock:stream:{}", stream_id)
}
fn control_subject(stream_id: &str) -> String {
    format!("control.stream.shutdown.{}", stream_id)
}

#[derive(Clone)]
pub struct DistributedSubscriptionService {
    redis_client: redis::Client,
    publisher: Arc<dyn RealtimePublisher>,
    subscriber: Arc<dyn RealtimeSubscriber>,
    provider: Arc<dyn StreamProvider>,
    instance_id: String,
}

impl DistributedSubscriptionService {
    pub fn new(
        redis_client: redis::Client,
        publisher: Arc<dyn RealtimePublisher>,
        subscriber: Arc<dyn RealtimeSubscriber>,
        provider: Arc<dyn StreamProvider>,
        instance_id: String,
    ) -> Self {
        Self {
            redis_client,
            publisher,
            subscriber,
            provider,
            instance_id,
        }
    }

    pub async fn subscribe(&self, stream_id: String) -> Result<Subscription, redis::RedisError> {
        let mut con = self.redis_client.get_multiplexed_async_connection().await?;
        let key = ref_count_key(&stream_id);

        let count: usize = con.incr(key, 1).await?;
        info!(stream_id = %stream_id, ref_count = count, "Consumer subscribed (distributed)");

        if count == 1 {
            info!(stream_id = %stream_id, "First consumer in cluster. Attempting to become leader...");
            let self_clone = self.clone();
            let stream_id_clone = stream_id.clone();
            tokio::spawn(async move { self_clone.try_become_leader(&stream_id_clone).await });
        }

        Ok(Subscription::new(stream_id, self.clone()))
    }

    async fn unsubscribe(&self, stream_id: &str) -> Result<(), redis::RedisError> {
        let mut con = self.redis_client.get_multiplexed_async_connection().await?;
        let key = ref_count_key(stream_id);

        let count: usize = con.decr(key, 1).await?;
        info!(stream_id = %stream_id, ref_count = count, "Consumer unsubscribed (distributed)");

        if count == 0 {
            info!(stream_id = %stream_id, "Last consumer in cluster. Publishing shutdown signal.");
            self.publisher
                .publish(&dexevents::StreamShutdown {
                    stream_id: stream_id.to_string(),
                })
                .await
                .unwrap_or_else(|e| error!("Failed to publish shutdown signal: {}", e));

            let _: () = con
                .del(&[ref_count_key(stream_id), lock_key(stream_id)])
                .await?;
        }
        Ok(())
    }

    async fn try_become_leader(&self, stream_id: &str) -> DexMessagingResult<()> {
        let mut con = self
            .redis_client
            .get_multiplexed_async_connection()
            .await
            .context(RedisSnafu)?;

        let lock_key = lock_key(stream_id);
        let set_options = redis::SetOptions::default()
            .conditional_set(redis::ExistenceCheck::NX)
            .with_expiration(redis::SetExpiry::EX(LOCK_TTL_SECS));
        let result: bool = con
            .set_options(lock_key, &self.instance_id, set_options)
            .await
            .context(RedisSnafu)?;

        if !result {
            return Ok(());
        }

        let (abort_handle, abort_registration) = future::AbortHandle::new_pair();

        let provider_clone = self.provider.clone();
        let publisher_clone = self.publisher.clone();
        let redis_client_clone = self.redis_client.clone();
        let stream_id_for_task = stream_id.to_string();

        let provider_task = tokio::spawn(future::Abortable::new(
            async move {
                Self::run_provider_and_heartbeat(
                    stream_id_for_task,
                    provider_clone,
                    publisher_clone,
                    redis_client_clone,
                )
                .await;
            },
            abort_registration,
        ));

        let subscription_service_clone = self.subscriber.clone();
        let stream_id_clone = stream_id.to_string();
        tokio::spawn(async move {
            let subject = control_subject(&stream_id_clone);
            let handler = Box::new(move |_payload: Vec<u8>| {
                info!(stream_id = %stream_id_clone, "Shutdown signal received. Aborting leader tasks.");
                abort_handle.abort();
                Ok(())
            });
            let _ = subscription_service_clone
                .subscribe_raw(subject, handler)
                .await;
        });

        let _ = provider_task.await;
        info!(stream_id = %stream_id, "Leader tasks have terminated.");

        Ok(())
    }

    async fn run_provider_and_heartbeat(
        stream_id: String,
        provider: Arc<dyn StreamProvider>,
        publisher: Arc<dyn RealtimePublisher>,
        redis_client: redis::Client,
    ) {
        let provider_fut = provider.start_stream(stream_id.clone(), publisher.clone());

        let heartbeat_fut = async {
            let mut interval = tokio::time::interval(Duration::from_secs(HEARTBEAT_INTERVAL_SECS));
            let mut con = match redis_client.get_multiplexed_async_connection().await {
                Ok(con) => con,
                Err(e) => {
                    error!(error = %e, "Heartbeat task failed to get Redis connection. Terminating.");
                    return;
                }
            };

            loop {
                interval.tick().await;

                let ref_count_key = ref_count_key(&stream_id);
                let count_result: redis::RedisResult<Option<usize>> = con.get(&ref_count_key).await;

                let count = match count_result {
                    Ok(Some(c)) => c,
                    Ok(None) => 0,
                    Err(e) => {
                        error!(stream_id = %stream_id, error = %e, "Heartbeat failed to read ref_count. Terminating for safety.");
                        break;
                    }
                };

                let lock_key = lock_key(&stream_id);
                if count == 0 {
                    warn!(stream_id = %stream_id, "Heartbeat check: ref_count is zero. Self-terminating zombie leader.");
                    let _: redis::RedisResult<()> = con.del(&lock_key).await;
                    break;
                }

                let renewed: redis::RedisResult<bool> =
                    con.expire(&lock_key, LOCK_TTL_SECS as i64).await;
                if let Err(e) = renewed {
                    error!(stream_id = %stream_id, error = %e, "Heartbeat failed to renew lock. Terminating.");
                    break;
                }
                info!(stream_id = %stream_id, "Heartbeat: Lock renewed successfully.");
            }
        };

        tokio::select! {
            _ = provider_fut => { warn!(stream_id = %stream_id, "Provider stream task finished unexpectedly."); },
            _ = heartbeat_fut => { info!(stream_id = %stream_id, "Heartbeat task finished. Leader is shutting down."); },
        }
    }
}

pub struct Subscription {
    stream_id: String,
    service: DistributedSubscriptionService,
}

impl Subscription {
    fn new(stream_id: String, service: DistributedSubscriptionService) -> Self {
        Self { stream_id, service }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        let service = self.service.clone();
        let stream_id = self.stream_id.clone();
        tokio::spawn(async move { service.unsubscribe(&stream_id).await });
    }
}
