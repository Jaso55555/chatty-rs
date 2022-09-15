// let mut socket = TcpStream::connect("127.0.0.1:5678").expect("Could not connect to server");
// loop {
//     match Message::try_read(&mut socket) {
//         None => {}
//         Some(msg) => {println!("{}", msg)}
//     }
//
//     sleep(Duration::from_millis(100))
// }

pub mod error;

use std::io::Write;
use std::net::TcpStream;
use rmp_serde::Serializer;
use serde::Serialize;
use common::message::Message;
use crate::net::error::NetCodeErr;

pub struct NetCode {
    socket: TcpStream
}

impl NetCode {
    pub fn init() -> Result<Self, NetCodeErr> {
        let socket = match TcpStream::connect("127.0.0.1:5678") {
            Ok(socket) => socket,
            Err(_error) => return Err(NetCodeErr::CouldNotConnect)
        };

        match socket.set_nonblocking(true) {
            Err(_error) => return Err(NetCodeErr::CouldNotSetNonBlocking),
            _ => {}
        };

        Ok( Self {
            socket
        })
    }

    pub fn check_for_messages(&mut self) -> Option<Message> {
        match Message::try_read(&mut self.socket) {
            None => None,
            Some(msg) => Some(msg)
        }
    }

    pub fn send_message(&mut self, msg: &Message) -> Result<(), NetCodeErr> {
        let mut buf = Vec::new();

        msg.serialize( &mut Serializer::new(&mut buf) ).expect("Could not serialize message");

        match self.socket.write_all(&buf[..]) {
            Ok(_) => Ok(()),
            Err(_error) => Err(NetCodeErr::MessageSendFailed)
        }
    }
}