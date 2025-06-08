use crate::error::VariableSnafu;
use crate::Result;
use snafu::ResultExt;
use std::env;
#[derive(Debug, Clone)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Default)]
pub struct IggyConfig {
    pub server_address: String,
    pub connect_timeout_ms: u64,
    pub request_timeout_ms: u64,
    pub reconnect_interval_ms: u64,
    pub max_reconnect_retries: Option<u32>,
    pub login_credentials: Option<LoginCredentials>,
}

impl IggyConfig {
    pub fn from_env() -> Result<Self> {
        let server_address = env::var("IGGY_SERVER_ADDRESS").context(VariableSnafu {
            message: "IGGY_SERVER_ADDRESS".to_string(),
        })?;

        Ok(Self {
            server_address,
            ..Default::default()
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct NatsConfig {
    pub server_address: String,
    pub connect_timeout_ms: u64,
    pub request_timeout_ms: u64,
    pub reconnect_interval_ms: u64,
    pub max_reconnect_retries: Option<u32>,
}

impl NatsConfig {
    pub fn from_env() -> Result<Self> {
        let server_address = env::var("NATS_URL").context(VariableSnafu {
            message: "NATS_URL".to_string(),
        })?;

        Ok(Self {
            server_address,
            ..Default::default()
        })
    }
}
