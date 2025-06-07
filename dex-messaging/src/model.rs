use iggy::identifier::Identifier;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Event: Serialize + DeserializeOwned + Send + Sync + 'static {
    fn stream_id() -> Identifier;

    fn topic_id() -> Identifier;

    fn event_name() -> &'static str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapCompleted {
    pub transaction_id: String,
    pub user_id: u64,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: f64,
}

impl Event for SwapCompleted {
    fn stream_id() -> Identifier {
        Identifier::numeric(1).unwrap() // Stream "dex_events"
    }

    fn topic_id() -> Identifier {
        Identifier::numeric(1).unwrap() // Topic "swaps"
    }

    fn event_name() -> &'static str {
        "SwapCompleted"
    }
}

// Você pode adicionar quantos eventos quiser...
#[derive(Debug, Serialize, Deserialize)]
pub struct UserScored {
    pub user_id: u64,
    pub new_score: u32,
}

impl Event for UserScored {
    fn stream_id() -> Identifier {
        Identifier::numeric(1).unwrap() // Stream "dex_events"
    }

    fn topic_id() -> Identifier {
        Identifier::numeric(2).unwrap() // Topic "scores"
    }

    fn event_name() -> &'static str {
        "UserScored"
    }
}
