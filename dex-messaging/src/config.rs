use crate::error::{ConfigSnafu, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct MessagingConfig {
    pub iggy_server_address: String,
    pub connect_timeout_ms: u64,
    pub request_timeout_ms: u64,
    pub reconnect_interval_ms: u64,
    pub max_reconnect_retries: Option<u32>,
    pub login_credentials: Option<LoginCredentials>,
}

impl MessagingConfig {
    pub fn from_env() -> Result<Self> {
        let iggy_server_address = env::var("IGGY_SERVER_ADDRESS").map_err(|_| {
            ConfigSnafu {
                message: "IGGY_SERVER_ADDRESS".to_string(),
            }
            .build()
        })?;
        let username = env::var("IGGY_USERNAME").map_err(|_| {
            ConfigSnafu {
                message: "IGGY_USERNAME".to_string(),
            }
            .build()
        })?;
        let password = env::var("IGGY_PASSWORD").map_err(|_| {
            ConfigSnafu {
                message: "IGGY_PASSWORD".to_string(),
            }
            .build()
        })?;
        let login_credentials = LoginCredentials { username, password };
        Ok(Self {
            iggy_server_address,
            connect_timeout_ms: 1,
            max_reconnect_retries: None,
            reconnect_interval_ms: 3,
            request_timeout_ms: 1,
            login_credentials: Some(login_credentials),
        })
    }
}
