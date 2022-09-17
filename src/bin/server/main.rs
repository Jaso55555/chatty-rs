use std::fs::File;
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::thread::spawn;

use chrono::Utc;

use log::info;
use common::message::Message;
use common::net::active::ActivePacket;
use common::server_config::ServerConfig;

pub mod user;
use user::User;

fn main() {
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
            File::create(
                format!("logs\\server-{}.txt", Utc::now().format("%d-%m-%Y-%H-%M"))
            ).expect("Could not create log")
        )
    ]).expect("Could not init logger");

    info!("Running server!");

    let (config, new_config) = ServerConfig::load();

    info!("Config loaded, was new config created: {}", new_config);

    let listener = TcpListener::bind("127.0.0.1:5678").expect("Could not bind to IP");
    let (to_setup_tx, to_setup_rx) = channel::<User>();
    let (to_active_tx, to_active_rx) = channel::<User>();

    let _listener_thread = spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(socket) => {
                    let user = match User::new(socket) {
                        Err(error) => {
                            info!("{}", error);
                            continue
                        }
                        Ok(user) => user
                    };

                    to_setup_tx.send(user).expect("Could not send user")
                }
                Err(_) => {}
            }
        }
    });

    let _setup_thread = spawn(move || {
        let mut users: Vec<User> = Vec::new();
        loop {
            users.append(&mut to_setup_rx.try_iter().collect());

            let mut i = 0;
            while i < users.len() {
                match users[i].setup_behave() {
                    Some(Ok(_)) => {
                        let mut user = users.remove(i);

                        user.send_packets(&vec![
                            ActivePacket::SystemMessage(Message {
                                sender: config.name.clone(),
                                content: config.motd.clone(),
                                timestamp: Utc::now(),
                                color: config.color
                            })
                        ]);

                        to_active_tx.send(user).expect("Could not send user")
                    }
                    Some(Err(error)) => {
                        info!("Client disconnected: {}", error);
                        users.remove(i).close(format!("Disconnected: {}", error));
                    }
                    None => {
                        i += 1;
                    }
                }
            }
        }
    });

    let _active_thread = spawn(move || {
        let mut users: Vec<User> = Vec::new();
        loop {
            users.append(&mut to_active_rx.try_iter().collect());

            let mut outbound_queue = Vec::new();

            let mut i = 0;
            while i < users.len() {
                match users[i].behave() {
                    Some(Ok(msg)) => {
                        match msg {
                            ActivePacket::Shutdown { reason } => {
                                info!("Client disconnected: {}", reason);
                                users.remove(i).close("Client disconnect");
                                continue
                            }
                            // All other packets get forwarded back out
                            packet => {
                                match &packet {
                                    ActivePacket::Message(msg)
                                    | ActivePacket::SystemMessage(msg) => {
                                        info!("{}", msg)
                                    }
                                    ActivePacket::Shutdown { .. } => {}
                                }
                                outbound_queue.push(packet)
                            }
                        }
                    }
                    Some(Err(error)) => {
                        info!("Client disconnected: {}", error);
                        users.remove(i).close(format!("Disconnected: {}", error));
                        continue
                    }
                    _ => {}
                }
                i += 1;
            }
            outbound_queue.push(ActivePacket::Shutdown { reason: "Test reason".to_string() });
            // Send outbound queue to all users
            for user in users.iter_mut() {
                user.send_packets(&outbound_queue);
            }
        }
    });

    loop {}
}
