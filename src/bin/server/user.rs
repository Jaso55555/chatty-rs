use std::io;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use log::{error, info, warn};
use rmp_serde::decode::Error;
use serde::de::DeserializeOwned;
use common::net::error::NetCodeErr;
use common::net::init::{ConnectionInit, FinalHandshake};
use common::client_config::ClientConfig;
use common::net::active::ActivePacket;
use common::utils::{await_object, serialize_rmp, write_obj_to_socket};
use crate::user::UserState::{Closed, Closing};

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
            UserState::Active | Closing { .. } | Closed => {
                unreachable!()
            }
        }
    }

    pub fn behave(&mut self) -> Option<Result<ActivePacket, NetCodeErr>> {
        match await_object::<ActivePacket>(&mut self.socket) {
            Some(msg) => Some(Ok(msg)),
            None => None,
        }
    }

    pub fn send_packets(&mut self, packets: &Vec<ActivePacket>) -> Vec<(io::Error, ActivePacket)> {
        let mut errors = Vec::new();

        for item in packets.iter() {
            match write_obj_to_socket(&mut self.socket, item) {
                Ok(_) => {},
                Err(error) => {
                    errors.push((error, (*item).clone()))
                }
            }
        }

        errors
    }

    fn await_object<T: DeserializeOwned>(&mut self) -> Result<T, Error> {
        rmp_serde::from_read::<&mut TcpStream, T>(&mut self.socket)
    }

    pub fn close<T: ToString>(mut self, reason: T) {
        self.state = Closing { reason: reason.to_string() }
    }
}

impl Drop for User {
    fn drop(&mut self) {
        if let Err(e) = self.socket.set_nonblocking(false) {
            warn!("Could not set blocking mode on closing connection: {}", e)
        };
        match &self.state {
            UserState::WaitingForConfig | Closed => {}
            UserState::Active => {
                self.send_packets(&vec![
                    ActivePacket::Shutdown {
                        reason: "Server closed connection unexpectedly".to_string()
                    }
                ]);
            }
            Closing { reason } => {
                ActivePacket::Shutdown {
                    reason: reason.clone()
                };
                if let Err(e) = self.socket.shutdown(Shutdown::Both) {
                    error!("Could not shutdown socket: {}", e)
                }
            }
        }
    }
}

enum UserState {
    // Unauthenticated, TODO
    WaitingForConfig,
    Active,
    Closing { reason: String },
    Closed
}