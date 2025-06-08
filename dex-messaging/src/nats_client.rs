use crate::config::NatsConfig;
use crate::error::{NatsConnectSnafu, NatsPublishSnafu, NatsSnafu};
use crate::traits::DynRealtime;
use crate::DexMessagingResult;
use async_trait::async_trait;
use futures::future;
use snafu::prelude::*;

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
impl DynRealtime for NatsMessagingClient {
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
