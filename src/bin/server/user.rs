use std::io::Write;
use std::net::{Shutdown, TcpStream};
use log::{error, info, warn};
use rmp_serde::decode::Error;
use serde::de::DeserializeOwned;
use common::net::error::NetCodeErr;
use common::net::init::{ConnectionInit, FinalHandshake};
use common::client_config::ClientConfig;
use common::message::Message;
use common::utils::{serialize_rmp, write_obj_to_socket};

pub struct User {
    socket: TcpStream,
    state: UserState,
    config: Option<ClientConfig>
}

impl User {
    /// On error the user is disconnected
    // Connection -> Auth -> Get config -> Send Channel info -> Normal operation
    pub fn new(mut socket: TcpStream) -> Result<Self, NetCodeErr> {
        info!("User connected");
        socket.write_all(&serialize_rmp(ConnectionInit::Connected).unwrap()[..])?;

        match socket.set_nonblocking(true) {
            // if cannot set non-blocking,
            Err(_error) => {
                match socket.write_all(
                    &serialize_rmp(ConnectionInit::Failed).unwrap()[..]
                ) {
                    Err(_error) => return Err(NetCodeErr::MessageSendFailed),
                    _ => {}
                }

                socket.shutdown(Shutdown::Both)?;

                return Err(NetCodeErr::CouldNotSetNonBlocking)
            }
            _ => {}
        }

        socket.set_nodelay(true)?;

        Ok(Self {
            socket,
            // Add auth later
            state: UserState::WaitingForConfig,
            config: None
        })
    }

    pub fn setup_behave(&mut self) -> Option<Result<(), NetCodeErr>> {
        match self.state {
            UserState::WaitingForConfig => {
                match self.await_object::<ClientConfig>() {
                    Ok(config) => {
                        info!("Username {} connected", config.username);
                        self.config = Some(config);

                        match write_obj_to_socket(&mut self.socket, FinalHandshake::Complete) {
                            Ok(_) => {}
                            Err(error) => {
                                error!("Could not send back final handshake: {error:?}");
                                return Some(Err(NetCodeErr::from(error)))
                            }
                        }

                        self.state = UserState::Active;

                        info!("Got config!");

                        Some(Ok(()))
                    }
                    Err(err) => {
                        warn!("Waiting for config warning! {err}");
                        None
                    }
                }
            }
            UserState::Active => {
                unreachable!()
            }
        }
    }

    pub fn behave(&mut self) -> Option<Result<(), NetCodeErr>> {
        None
    }

    pub fn send_message(&mut self, _messages: &Vec<Message>) {
        todo!()
    }

    fn await_object<T: DeserializeOwned>(&mut self) -> Result<T, Error> {
        rmp_serde::from_read::<&mut TcpStream, T>(&mut self.socket)
    }

    pub fn close(self) {
        todo!()
    }

    // fn try_recv_config(&mut self) -> Option<Result<(), NetCodeErr>> {
    //     let mut buf = Vec::new();
    //
    //     self.socket.read_to_end(&mut buf).ok()?;
    //
    //     match deserialize_rmp::<ClientConfig>(buf) {
    //         Ok(config) => self.config = Some(config),
    //         Err(_) => return Some(Err(NetCodeErr::BadPacket))
    //     }
    //
    //     return None
    // }
}

enum UserState {
    // Unauthenticated, TODO
    WaitingForConfig,
    Active
}