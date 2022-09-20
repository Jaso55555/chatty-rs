use crate::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ActivePacket {
    Message(Message),
    SystemMessage(Message),
    Shutdown { reason: String },
}
