use std::fmt::{Display, Formatter};
use std::io::Read;
use chrono::{DateTime, Utc};
use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub color: [u8; 3]
}

impl Message {
    pub fn try_read<T: Read>(mut source: &mut T) -> Option<Self> {
        return match rmp_serde::from_read::<&mut T, Self>(&mut source) {
            Ok(message) => Some(message),
            Err(_) => None
        }
    }

    pub fn new_system_message<T: ToString>(content: T) -> Self {
        Self {
            sender: "[CLIENT]".to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
            color: [255, 247, 0]
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.sender, self.content)
    }
}