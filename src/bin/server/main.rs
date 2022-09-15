use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use chrono::Utc;

use serde::Serialize;
use rmp_serde::Serializer;

use common::message::Message;

pub mod user;


fn main() {
    println!("Running server!");

    let listener = TcpListener::bind("127.0.0.1:5678").expect("Could not bind to IP");
    let mut users: Vec<TcpStream> = Vec::new();

    let _listener_thread = spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(mut stream) => {

                    let msg = Message {
                        sender: "%SRV%".to_string(),
                        content: "Connected!".to_string(),
                        timestamp: Utc::now(),
                        color: [255, 247, 0]
                    };
                    let mut buf = Vec::new();

                    msg.serialize( &mut Serializer::new(&mut buf) ).expect("Could not serialize message");

                    stream.write_all(
                        buf.as_slice()
                    ).expect("Could not send message");

                    users.push(stream)
                }
                Err(_) => {}
            }
        }
    });

    loop {}
}
