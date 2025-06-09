use crate::model::dexevents::PriceUpdate;
use crate::model::{PersistentEvent, RealtimeEvent, RealtimeEventSubject};
use crate::DexMessagingResult;
use async_trait::async_trait;

#[async_trait]
pub trait Persistent: Send + Sync {
    async fn publish<E: PersistentEvent>(&self, event: &E) -> DexMessagingResult<()>;

    async fn publish_batch<E: PersistentEvent>(&self, events: &[E]) -> DexMessagingResult<()>;

    async fn consume<E, F>(&self, subject: &str, handler: F) -> DexMessagingResult<()>
    where
        E: PersistentEvent,
        F: FnMut(E) -> DexMessagingResult<()> + Send;
}

/// Dyn-compatible trait for realtime subscribing
#[async_trait]
pub trait RealtimeSubscriber: Send + Sync {
    async fn subscribe_raw(
        &self,
        subject: String,
        handler: Box<dyn Fn(Vec<u8>) -> DexMessagingResult<()> + Send + Sync>,
    ) -> DexMessagingResult<()>;
}

/// Extension trait to provide generic methods for RealtimeSubscriber
#[async_trait]
pub trait RealtimeSubscriberExt {
    async fn subscribe<E, F>(
        &self,
        subject: RealtimeEventSubject,
        handler: F,
    ) -> DexMessagingResult<()>
    where
        E: RealtimeEvent,
        F: FnMut(E) -> DexMessagingResult<()> + Send;
}

/// Dyn-compatible trait for realtime messaging
/// This trait uses serialized data internally to be object-safe while providing a generic interface
#[async_trait]
pub trait RealtimePublisher: Send + Sync {
    async fn publish_raw(&self, subject: String, payload: Vec<u8>) -> DexMessagingResult<()>;
    async fn publish_batch_raw(&self, messages: &[(String, Vec<u8>)]) -> DexMessagingResult<()>;
}

/// Extension trait to provide generic methods for DynRealtime
#[async_trait]
pub trait RealtimePublisherExt {
    async fn publish<E: RealtimeEvent>(&self, event: &E) -> DexMessagingResult<()>;
    async fn publish_batch<E: RealtimeEvent>(&self, events: &[E]) -> DexMessagingResult<()>;
}

#[async_trait]
impl<T: RealtimePublisher + ?Sized> RealtimePublisherExt for T {
    async fn publish<E: RealtimeEvent>(&self, event: &E) -> DexMessagingResult<()> {
        let subject = event.subject();
        let payload = event.encode_to_vec();
        self.publish_raw(subject, payload).await
    }

    async fn publish_batch<E: RealtimeEvent>(&self, events: &[E]) -> DexMessagingResult<()> {
        let messages: Vec<(String, Vec<u8>)> = events
            .iter()
            .map(|event| (event.subject(), event.encode_to_vec()))
            .collect();
        self.publish_batch_raw(&messages).await
    }
}
