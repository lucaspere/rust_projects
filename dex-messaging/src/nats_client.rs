use crate::config::NatsConfig;
use crate::error::{NatsConnectSnafu, NatsPublishSnafu, NatsSnafu, NatsSubscribeSnafu};
use crate::model::{RealtimeEvent, RealtimeEventSubject};
use crate::traits::{RealtimePublisher, RealtimeSubscriber};
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
impl RealtimeSubscriber for NatsMessagingClient {
    async fn subscribe_raw(
        &self,
        subject: String,
        handler: Box<dyn Fn(Vec<u8>) -> DexMessagingResult<()> + Send + Sync>,
    ) -> DexMessagingResult<()> {
        info!(subject = %subject, "Subscribing to realtime subject");
        let mut sub = self
            .client
            .subscribe(subject.clone())
            .await
            .context(NatsSubscribeSnafu)
            .context(NatsSnafu)?;

        while let Some(msg) = sub.next().await {
            if let Err(e) = handler(msg.payload.to_vec()) {
                error!(subject = %subject, error = %e, "Error processing realtime event");
            }
        }

        Ok(())
    }
}

#[async_trait]
impl RealtimePublisher for NatsMessagingClient {
    async fn publish_raw(&self, subject: String, payload: Vec<u8>) -> DexMessagingResult<()> {
        self.client
            .publish(subject, payload.into())
            .await
            .context(NatsPublishSnafu)
            .context(NatsSnafu)?;

        Ok(())
    }

    async fn publish_batch_raw(&self, messages: &[(String, Vec<u8>)]) -> DexMessagingResult<()> {
        future::join_all(
            messages
                .iter()
                .map(|(subject, payload)| self.publish_raw(subject.clone(), payload.clone())),
        )
        .await;

        Ok(())
    }
}
