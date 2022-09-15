use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub username: String,
    pub user_color: [u8; 3]
}

impl Config {
    pub fn load() -> Self {
        let file = File::open("config/client_config.json").expect("Config file does not exist (config/client_config.json)");

        serde_json::from_reader(file).expect("Invalid config file")
    }
}