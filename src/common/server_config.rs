use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::config::{load_config, write_config};

const SERVER_CONFIG_PATH: &'static str = "server_config.json";

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub motd: String,
    pub color: [u8; 3],
    pub tickrate: u64,
    pub ip: String
}

impl ServerConfig {
    /// Output.1: Was new config created
    pub fn load() -> (Self, bool) {
        match load_config(PathBuf::from(SERVER_CONFIG_PATH)) {
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
        write_config(PathBuf::from(SERVER_CONFIG_PATH), self)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "Default Server Name".to_string(),
            motd: "Default MOTD".to_string(),
            color: [255, 247, 0],
            tickrate: 50,
            ip: "127.0.0.1:5678".to_string()
        }
    }
}
