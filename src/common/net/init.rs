use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize)]
pub enum ConnectionInit {
    Connected,
    Failed
}

#[derive(Serialize, Deserialize)]
pub enum FinalHandshake {
    Complete
}