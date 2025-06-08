use crate::error::ErrorSnafu;
use async_trait::async_trait;
use iggy::identifier::Identifier;
use serde::{Serialize, de::DeserializeOwned};

pub mod dexevents {
    include!(concat!(env!("OUT_DIR"), "/dexevents.rs"));
}

pub(crate) type DexMessagingResult<T> = std::result::Result<T, ErrorSnafu>;

pub trait Event: Serialize + DeserializeOwned + Send + Sync + 'static {
    fn stream_id() -> Identifier;

    fn topic_id() -> Identifier;

    fn event_name() -> &'static str;
}

pub trait PersistentEvent: prost::Message + Default + Send + Sync + 'static {
    fn stream_id() -> Identifier;
    fn topic_id() -> Identifier;
    fn event_name() -> &'static str {
        std::any::type_name::<Self>()
            .rsplit("::")
            .next()
            .unwrap_or("UnknownEvent")
    }
}

impl PersistentEvent for dexevents::SwapCompleted {
    fn stream_id() -> Identifier {
        Identifier::numeric(1).unwrap()
    }

    fn topic_id() -> Identifier {
        Identifier::numeric(1).unwrap()
    }

    fn event_name() -> &'static str {
        "swap_completed"
    }
}

#[async_trait]
pub trait RealtimeEvent: prost::Message + Default + Send + Sync + 'static {
    fn subject(&self) -> String;
}

#[async_trait]
impl RealtimeEvent for dexevents::PriceUpdate {
    fn subject(&self) -> String {
        format!("prices.{}", self.token_pair)
    }
}
