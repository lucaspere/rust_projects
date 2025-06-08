use crate::{prelude::*, DexMessagingResultAsync};
use async_trait::async_trait;
use chrono::Utc;
use std::{sync::Arc, time::Duration};
use tokio::time::interval;

#[async_trait]
pub trait StreamProvider: Send + Sync + 'static {
    async fn start_stream(
        &self,
        stream_id: String,
        publisher: Arc<dyn DynRealtime>,
    ) -> DexMessagingResultAsync<()>;
}

pub struct MockStreamProvider;

#[async_trait]
impl StreamProvider for MockStreamProvider {
    async fn start_stream(
        &self,
        stream_id: String,
        publisher: Arc<dyn DynRealtime>,
    ) -> DexMessagingResultAsync<()> {
        let mut price = 100.0;
        let mut timer = interval(Duration::from_secs(1));

        loop {
            timer.tick().await;
            price += 0.5;

            let event = PriceUpdate {
                token_pair: stream_id.clone(),
                price,
                timestamp: Utc::now().timestamp_millis() as u64,
            };

            publisher.publish(&event).await?;
        }
    }
}
