// Em shared-messaging/src/nats_client.rs
use crate::config::NatsConfig;
use crate::error::{NatsConnectSnafu, NatsPublishSnafu, NatsSnafu, NatsSubscribeSnafu};
use crate::model::{RealtimeEvent, RealtimeEventSubject};
use crate::traits::Realtime;
use crate::DexMessagingResult;
use async_trait::async_trait;
use futures::{future, StreamExt};
use snafu::prelude::*;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct NatsMessagingClient {
    client: async_nats::Client,
}

impl NatsMessagingClient {
    pub async fn new(config: NatsConfig) -> DexMessagingResult<Self> {
        let client = async_nats::connect(&config.server_address)
            .await
            .context(NatsConnectSnafu)
            .context(NatsSnafu)?;

        Ok(Self { client })
    }
}

#[async_trait]
impl Realtime for NatsMessagingClient {
    async fn publish<E: RealtimeEvent>(&self, event: &E) -> DexMessagingResult<()> {
        let subject = event.subject();
        let payload = event.encode_to_vec();
        self.client
            .publish(subject, payload.into())
            .await
            .context(NatsPublishSnafu)
            .context(NatsSnafu)?;

        Ok(())
    }

    async fn publish_batch<E: RealtimeEvent>(&self, events: &[E]) -> DexMessagingResult<()> {
        future::join_all(events.iter().map(|event| self.publish(event))).await;

        Ok(())
    }

    async fn subscribe<E, F>(
        &self,
        subject: RealtimeEventSubject,
        mut handler: F,
    ) -> DexMessagingResult<()>
    where
        E: RealtimeEvent,
        F: FnMut(E) -> DexMessagingResult<()> + Send,
    {
        info!(subject = %subject, "Subscribing to realtime subject");
        let mut sub = self
            .client
            .subscribe(subject.to_string())
            .await
            .context(NatsSubscribeSnafu)
            .context(NatsSnafu)?;

        while let Some(msg) = sub.next().await {
            match E::decode(&msg.payload[..]) {
                Ok(event) => {
                    if let Err(e) = handler(event) {
                        error!(subject = %msg.subject, error = %e, "Error processing realtime event");
                    }
                }
                Err(e) => {
                    warn!(subject = %msg.subject, error = %e, "Failed to deserialize realtime event payload");
                }
            }
        }

        Ok(())
    }
}
