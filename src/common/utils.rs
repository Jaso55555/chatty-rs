use std::io;
use std::io::Write;
use std::net::TcpStream;

use rmp_serde::decode::Error as DeError;
use rmp_serde::encode::Error as SeError;
use rmp_serde::Serializer;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn serialize_rmp<I: Serialize>(obj: I) -> Result<Vec<u8>, SeError> {
    let mut buf = Vec::new();

    obj.serialize(&mut Serializer::new(&mut buf))?;

    Ok(buf)
}

pub fn deserialize_rmp<O: DeserializeOwned>(buf: Vec<u8>) -> Result<O, DeError> {
    rmp_serde::from_slice(&buf[..])
}

pub fn write_obj_to_socket<I: Serialize>(socket: &mut TcpStream, obj: I) -> io::Result<()> {
    let buf = serialize_rmp(obj).expect("Could not serialize object");

    socket.write_all(&buf[..])
}

pub fn await_object<T: DeserializeOwned>(socket: &mut TcpStream) -> Option<T> {
    match rmp_serde::from_read::<&mut TcpStream, T>(socket) {
        Ok(obj) => Some(obj),
        Err(_) => None,
    }
}
