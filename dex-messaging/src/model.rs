use iggy::identifier::Identifier;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Um trait que todo evento publicável no Iggy deve implementar.
/// Fornece metadados sobre onde o evento deve ser armazenado.
pub trait Event: Serialize + DeserializeOwned + Send + Sync + 'static {
    /// O identificador do Stream onde este evento será salvo.
    fn stream_id() -> Identifier;

    /// O identificador do Topic onde este evento será salvo.
    fn topic_id() -> Identifier;

    /// O nome do evento, usado para logs.
    fn event_name() -> &'static str;
}

// --------- Exemplo de Implementação de um Evento ---------
// Cada serviço pode definir seus próprios eventos, mas todos implementarão o trait Event.

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
