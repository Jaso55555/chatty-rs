// let mut socket = TcpStream::connect("127.0.0.1:5678").expect("Could not connect to server");
// loop {
//     match Message::try_read(&mut socket) {
//         None => {}
//         Some(msg) => {println!("{}", msg)}
//     }
//
//     sleep(Duration::from_millis(100))
// }

use std::io::Write;
use std::net::TcpStream;
use rmp_serde::Serializer;
use serde::Serialize;
use serde::de::DeserializeOwned;

use common::message::Message;
use common::net::error::NetCodeErr;
use common::net::init::{ConnectionInit, FinalHandshake};
use common::client_config::ClientConfig;
use common::utils::write_obj_to_socket;

#[derive(PartialEq)]
pub enum NetCodeState {
    // Waiting for
    WaitingForInitConfirm,
    WaitingForFinalHandshake,
    Active
}

pub struct NetCode {
    socket: TcpStream,
    state: NetCodeState
}

impl NetCode {
    pub fn init(_config: &ClientConfig) -> Result<Self, NetCodeErr> {
        let socket = match TcpStream::connect("127.0.0.1:5678") {
            Ok(socket) => socket,
            Err(_error) => return Err(NetCodeErr::CouldNotConnect)
        };

        socket.set_nonblocking(true)?;
        socket.set_nodelay(true)?;

        Ok(Self {
            socket,
            state: NetCodeState::WaitingForInitConfirm
        })
    }

    pub fn behave(&mut self, config: &ClientConfig) -> Option<Result<Message, NetCodeErr>> {
        match self.state {
            NetCodeState::WaitingForInitConfirm => {
                self.await_init_confirm(config)
            }
            NetCodeState::WaitingForFinalHandshake => {
                match self.await_object::<FinalHandshake>() {
                    Some(FinalHandshake::Complete) => {
                        self.state = NetCodeState::Active;
                        Some(Ok(Message::new_client_message("Finished Handshake")))
                    }
                    None => None
                }
            }
            NetCodeState::Active => {
                match self.check_for_messages() {
                    Some(msg) => Some(Ok(msg)),
                    None => None
                }
            }
        }
    }

    fn await_init_confirm(&mut self, config: &ClientConfig) -> Option<Result<Message, NetCodeErr>> {
        match self.await_object::<ConnectionInit>() {
            Some(_msg) => {

                match write_obj_to_socket(&mut self.socket, config) {
                    Ok(_) => {
                        self.state = NetCodeState::WaitingForFinalHandshake
                    }
                    Err(_) => return Some(Err(NetCodeErr::UnknownError))
                }
                Some(Ok(Message::new_client_message("Sending Config")))
            }
            None => None
        }
    }

    fn await_object<T: DeserializeOwned>(&mut self) -> Option<T> {
        match rmp_serde::from_read::<&mut TcpStream, T>(&mut self.socket) {
            Ok(obj) => Some(obj),
            Err(_) => None
        }
    }

    fn check_for_messages(&mut self) -> Option<Message> {
        match Message::try_read(&mut self.socket) {
            None => None,
            Some(msg) => Some(msg)
        }
    }

    pub fn check_state(&self) -> &NetCodeState {
        &self.state
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