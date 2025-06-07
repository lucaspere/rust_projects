// shared-messaging/src/error.rs
use snafu::prelude::*;
use std::backtrace::Backtrace;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))] // Torna os seletores de contexto públicos
pub enum MessagingError {
    #[snafu(display("Failed to connect or interact with Iggy server"))]
    Iggy {
        source: iggy::error::IggyError,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to serialize or deserialize event payload"))]
    Serialization {
        source: serde_json::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("A required configuration variable is missing: {}", message))]
    Config {
        message: String,
        backtrace: Backtrace,
    },

    #[snafu(display("NATS client error"))]
    Nats {
        source: async_nats::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to publish event"))]
    NatsPublish {
        source: async_nats::PublishError,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to subscribe event"))]
    NatsSubscribe {
        source: async_nats::SubscribeError,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to connect to NATS server"))]
    NatsConnect {
        source: async_nats::ConnectError,
        backtrace: Backtrace,
    },
}

pub type Result<T> = std::result::Result<T, MessagingError>;
