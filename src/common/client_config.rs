use crate::config::{load_config, write_config};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CLIENT_CONFIG_PATH: &'static str = "client_config.json";

#[derive(Deserialize, Serialize)]
pub struct ClientConfig {
    pub username: String,
    pub user_color: [u8; 3],
    pub ip: String,
}

impl ClientConfig {
    /// Output.1: Was new config created
    pub fn load() -> (Self, bool) {
        match load_config(PathBuf::from(CLIENT_CONFIG_PATH)) {
            Ok(config) => (config, false),
            Err(_error) => {
                let mut config = Self::default();

                // Save default config
                config.write();

                (config, true)
            }
        }
    }

    pub fn write(&mut self) {
        write_config(PathBuf::from(CLIENT_CONFIG_PATH), self)
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            username: "Default Username".to_string(),
            user_color: [255, 255, 255],
            ip: "127.0.0.1:5678".to_string(),
        }
    }
}
