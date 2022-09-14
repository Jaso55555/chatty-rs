use std::io::Write;
use std::net::{Shutdown, TcpListener};
use std::thread::spawn;

use serde::Serialize;
use rmp_serde::Serializer;

use common::message::Message;


fn main() {
    println!("Running server!");

    let listener = TcpListener::bind("127.0.0.1:5678").expect("Could not bind to IP");
    // let users: Vec<TcpStream> = Vec::new();

    let _listener_thread = spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(mut stream) => {

                    let msg = Message {
                        sender: "%SRV%".to_string(),
                        content: "Connected!".to_string()
                    };
                    let mut buf = Vec::new();

                    msg.serialize( &mut Serializer::new(&mut buf) ).expect("Could not serialize message");

                    stream.write(
                        buf.as_slice()
                    ).expect("Could not send message");
                    stream.shutdown(Shutdown::Both).expect("Could not close connection")
                }
                Err(_) => {}
            }
        }
    });

    loop {}
}