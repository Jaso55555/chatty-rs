use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use serde::de::{DeserializeOwned, Error};
use serde::Serialize;
use serde_json::error::Result;

pub fn load_config<C: DeserializeOwned>(name: PathBuf) -> Result<C> {
    serde_json::from_reader(
        match File::open(PathBuf::from("config").join(name)) {
            Ok(file) => file,
            Err(_) => return Err(Error::custom("File does not exist"))
        }
    )
}

pub fn write_config<C: Serialize>(name: PathBuf, config: &C) {
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .open(
            PathBuf::from("config").join(name)
        ).expect("Could not write config");

    serde_json::to_writer(file, &config).expect("Failed to serialize config");
}