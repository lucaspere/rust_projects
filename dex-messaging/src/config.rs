use crate::error::{ConfigSnafu, MessagingError, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct MessagingConfig {
    pub iggy_server_address: String,
    pub connect_timeout_ms: u64,
    pub request_timeout_ms: u64,
    pub reconnect_interval_ms: u64,
    pub max_reconnect_retries: Option<u32>,
}

impl MessagingConfig {
    pub fn from_env() -> Result<Self> {
        let iggy_server_address = env::var("IGGY_SERVER_ADDRESS").map_err(|_| {
            ConfigSnafu {
                message: "IGGY_SERVER_ADDRESS".to_string(),
            }
            .build()
        })?;

        Ok(Self {
            iggy_server_address,
            connect_timeout_ms: 1,
            max_reconnect_retries: None,
            reconnect_interval_ms: 3,
            request_timeout_ms: 1,
        })
    }
}
