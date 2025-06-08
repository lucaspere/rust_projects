use snafu::prelude::*;
use std::{backtrace::Backtrace, env::VarError};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ErrorSnafu {
    #[snafu(display("Failed to encode event payload"))]
    Encode {
        source: prost::EncodeError,
        backtrace: Backtrace,
    },
    #[snafu(display("Failed to decode event payload"))]
    Decode {
        source: prost::DecodeError,
        backtrace: Backtrace,
    },
    #[snafu(display("NATS error"))]
    Nats {
        source: NatsError,
        backtrace: Backtrace,
    },
    #[snafu(display("Iggy error"))]
    Iggy {
        source: IggyError,
        backtrace: Backtrace,
    },

    #[snafu(display("{}", message))]
    Whatever {
        message: String,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[snafu(display("{}", message))]
    Variable { message: String, source: VarError },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum IggyError {
    #[snafu(display("Iggy client error"))]
    IggyClient {
        source: iggy::error::IggyError,
        backtrace: Backtrace,
    },

    #[snafu(display("A required configuration variable is missing: {}", message))]
    IggyConfig {
        message: String,
        backtrace: Backtrace,
    },
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum NatsError {
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
