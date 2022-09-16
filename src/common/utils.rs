use std::io;
use std::io::Write;
use std::net::TcpStream;

use rmp_serde::encode::Error as SeError;
use rmp_serde::decode::Error as DeError;
use rmp_serde::Serializer;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn serialize_rmp<I: Serialize>(obj: I) -> Result<Vec<u8>, SeError>{
    let mut buf = Vec::new();

    obj.serialize( &mut Serializer::new(&mut buf) )?;

    Ok(buf)
}

pub fn deserialize_rmp<O: DeserializeOwned>(buf: Vec<u8>) -> Result<O, DeError> {
    rmp_serde::from_slice(&buf[..])
}

pub fn write_obj_to_socket<I: Serialize>(socket: &mut TcpStream, obj: I) -> io::Result<()> {
    let buf = serialize_rmp(obj).expect("Could not serialize object");

    socket.write_all(
        &buf[..]
    )
}

// pub fn read_from_socket<O: DeserializeOwned>(socket: &mut TcpStream) -> Option<Result<O, DeError>> {
//     let mut buf = Vec::new();
//
//     match socket.read_to_end(&mut buf) {
//         Err(_) => return None,
//         _ => {}
//     };
//
//     Some(deserialize_rmp::<O>(buf))
// }