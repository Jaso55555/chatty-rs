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

use common::message::Message;
use common::net::error::NetCodeErr;
use common::net::init::{ConnectionInit, FinalHandshake};
use common::client_config::ClientConfig;
use common::net::active::ActivePacket;
use common::net::active::ActivePacket::SystemMessage;
use common::utils::{await_object, write_obj_to_socket};

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

impl Into<bool> for &NetCodeState {
    fn into(self) -> bool {
        match self {
            NetCodeState::Active => true,
            _ => false
        }
    }
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

    pub fn behave(&mut self, config: &ClientConfig) -> Option<Result<ActivePacket, NetCodeErr>> {
        match self.state {
            NetCodeState::WaitingForInitConfirm => {
                self.await_init_confirm(config)
            }
            NetCodeState::WaitingForFinalHandshake => {
                match await_object::<FinalHandshake>(&mut self.socket) {
                    Some(FinalHandshake::Complete) => {
                        self.state = NetCodeState::Active;
                        Some(Ok(SystemMessage(Message::new_system_message("Finished Handshake"))))
                    }
                    None => None
                }
            }
            NetCodeState::Active => {
                match self.check_for_packets() {
                    Some(msg) => Some(Ok(msg)),
                    None => None
                }
            }
        }
    }

    fn await_init_confirm(&mut self, config: &ClientConfig) -> Option<Result<ActivePacket, NetCodeErr>> {
        match await_object::<ConnectionInit>(&mut self.socket) {
            Some(_msg) => {

                match write_obj_to_socket(&mut self.socket, config) {
                    Ok(_) => {
                        self.state = NetCodeState::WaitingForFinalHandshake
                    }
                    Err(_) => return Some(Err(NetCodeErr::UnknownError))
                }
                Some(Ok(SystemMessage(Message::new_system_message("Sending Config"))))
            }
            None => None
        }
    }

    fn check_for_packets(&mut self) -> Option<ActivePacket> {
        await_object::<ActivePacket>(&mut self.socket)
    }

    pub fn check_state(&self) -> &NetCodeState {
        &self.state
    }

    pub fn send_packet(&mut self, msg: &ActivePacket) -> Result<(), NetCodeErr> {
        let mut buf = Vec::new();

        msg.serialize( &mut Serializer::new(&mut buf) ).expect("Could not serialize message");

        match self.socket.write_all(&buf[..]) {
            Ok(_) => Ok(()),
            Err(_error) => Err(NetCodeErr::MessageSendFailed)
        }
    }
}