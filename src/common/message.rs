use std::fmt::{Display, Formatter};
use std::io::Read;
use serde::{
    Deserialize,
    Serialize
};
use tui::widgets::ListItem;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub content: String
}

impl Message {
    pub fn try_read<T: Read>(mut source: &mut T) -> Option<Self> {
        return match rmp_serde::from_read::<&mut T, Self>(&mut source) {
            Ok(message) => Some(message),
            Err(_) => None
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.sender, self.content)
    }
}

pub fn collect_messages(list: &Vec<Message>) -> Vec<ListItem> {
    list.iter().map(|item| ListItem::new(format!("{}", item))).collect()
}