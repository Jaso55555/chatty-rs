use std::fs::File;
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::thread::spawn;

use chrono::Utc;

use log::info;

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
                        to_active_tx.send(users.remove(i)).expect("Could not send user")
                    }
                    Some(Err(error)) => {
                        info!("{error}");
                        users.remove(i).close()
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

        }
    });

    loop {}
}
