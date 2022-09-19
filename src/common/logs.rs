use std::fs;
use std::fs::File;
use std::path::Path;
use chrono::Utc;

pub fn client_log_init() {
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        make_log_file("Client")
    ).expect("Could not init logger");
}

pub fn server_log_init() {
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            make_log_file("Server")
        )
    ]).expect("Could not init logger");
}

fn make_log_file<T: ToString>(name: T) -> File {
    if !Path::new("logs").exists() {
        fs::create_dir(Path::new("logs")).expect("Could not create logs folder");
    }

    File::create(
        format!("logs/{}-{}.txt", name.to_string(), Utc::now().format("%d-%m-%Y-%H-%M"))
    ).expect("Could not create log")
}