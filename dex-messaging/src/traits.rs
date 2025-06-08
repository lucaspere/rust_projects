use crate::model::{DexMessagingResult, PersistentEvent, RealtimeEvent};
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

#[async_trait]
pub trait Realtime: Send + Sync {
    async fn publish<E: RealtimeEvent>(&self, event: &E) -> DexMessagingResult<()>;

    async fn subscribe<E, F>(&self, subject: &str, handler: F) -> DexMessagingResult<()>
    where
        E: RealtimeEvent,
        F: FnMut(E) -> DexMessagingResult<()> + Send;
}
