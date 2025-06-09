use std::{sync::Arc, time::Duration};

use chrono::Utc;
use futures::future;
use redis::AsyncCommands;
use snafu::ResultExt;
use tracing::{error, info, warn};

use crate::{
    error::RedisSnafu,
    model::RealtimeEventSubject,
    providers::StreamProvider,
    traits::{RealtimePublisher, RealtimeSubscriber},
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

pub struct DistributedSubscriptionService {
    redis_client: redis::Client,
    publisher: Arc<dyn RealtimePublisher>,
    subscription_service: Arc<dyn RealtimeSubscriber>,
    provider: Arc<dyn StreamProvider>,
    instance_id: String,
}

impl DistributedSubscriptionService {
    pub fn new(
        redis_client: redis::Client,
        publisher: Arc<dyn RealtimePublisher>,
        subscription_service: Arc<dyn RealtimeSubscriber>,
        provider: Arc<dyn StreamProvider>,
        instance_id: String,
    ) -> Self {
        Self {
            redis_client,
            publisher,
            subscription_service,
            provider,
            instance_id,
        }
    }

    pub async fn subscribe(&self, stream_id: String) -> Result<Subscription, redis::RedisError> {
        let mut con = self.redis_client.get_async_connection().await?;
        let key = ref_count_key(&stream_id);

        let count: usize = con.incr(key, 1).await?;
        info!(stream_id = %stream_id, ref_count = count, "Consumer subscribed (distributed)");

        if count == 1 {
            info!(stream_id = %stream_id, "First consumer in cluster. Attempting to become leader...");
            let self_clone = self.clone();
            let stream_id_clone = stream_id.clone();
            tokio::spawn(async move {
                self_clone.try_become_leader(&stream_id_clone).await;
            });
        }

        Ok(Subscription::new(stream_id, self.clone()))
    }

    async fn unsubscribe(&self, stream_id: &str) -> Result<(), redis::RedisError> {
        let mut con = self.redis_client.get_async_connection().await?;
        let key = ref_count_key(stream_id);

        let count: usize = con.decr(key, 1).await?;
        info!(stream_id = %stream_id, ref_count = count, "Consumer unsubscribed (distributed)");

        if count == 0 {
            info!(stream_id = %stream_id, "Last consumer in cluster. Publishing shutdown signal.");
            self.publisher
                .publish(&ShutdownSignal {
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
        let provider_task = tokio::spawn(future::Abortable::new(
            self.run_provider_and_heartbeat(stream_id.to_string()),
            abort_registration,
        ));

        let subscription_service_clone = self.subscription_service.clone();
        let stream_id_clone = stream_id.to_string();
        tokio::spawn(async move {
            let subject = control_subject(&stream_id_clone);
            let _ = subscription_service_clone.subscribe(RealtimeEventSubject::Custom(subject), move |_: ShutdownSignal| {
                info!(stream_id = %stream_id_clone, "Shutdown signal received. Aborting leader tasks.");
                abort_handle.abort();
                Ok(())
            }).await;
        });

        let _ = provider_task.await;
        info!(stream_id = %stream_id, "Leader tasks have terminated.");

        Ok(())
    }

    async fn run_provider_and_heartbeat(&self, stream_id: String) {
        let provider_clone = self.provider.clone();
        let publisher_clone = self.publisher.clone();

        let provider_fut = provider_clone.start_stream(stream_id.clone(), publisher_clone);

        let heartbeat_fut = async {
            let mut interval = tokio::time::interval(Duration::from_secs(HEARTBEAT_INTERVAL_SECS));
            let mut con: redis::aio::MultiplexedConnection = self
                .redis_client
                .get_multiplexed_async_connection()
                .await
                .context(RedisSnafu)?;
            let key = lock_key(&stream_id);
            loop {
                interval.tick().await;
                let _: Result<(), _> = con.expire(&key, LOCK_TTL_SECS as i64).await;
            }
        };

        tokio::select! {
            _ = provider_fut => { warn!(stream_id = %stream_id, "Provider stream task finished unexpectedly."); },
            _ = heartbeat_fut => { warn!(stream_id = %stream_id, "Heartbeat task finished unexpectedly."); },
        }
    }
}

pub struct Subscription {
    stream_id: String,
    service: DistributedSubscriptionService,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ShutdownSignal {
    stream_id: String,
}
#[async_trait::async_trait]
impl RealtimeEventSubject for ShutdownSignal {
    fn to_subject(&self) -> RealtimeEventSubject {
        RealtimeEventSubject::Custom(control_subject(&self.stream_id))
    }
}
