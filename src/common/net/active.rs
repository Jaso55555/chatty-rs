use serde::{
    Serialize,
    Deserialize
};
use crate::message::Message;

#[derive(Serialize, Deserialize, Clone)]
pub enum ActivePacket {
    Message (Message),
    SystemMessage (Message),
    Shutdown { reason: String }
}