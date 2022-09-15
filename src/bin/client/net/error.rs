use std::fmt::{Debug, Display, Formatter};
use std::io::Error;

pub enum NetCodeErr {
    CouldNotConnect,
    MessageSendFailed,
    CouldNotSetNonBlocking,
    UnknownError
}

impl NetCodeErr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NetCodeErr::CouldNotConnect => "Could not connect to server",
                NetCodeErr::MessageSendFailed => "Could not send message",
                NetCodeErr::CouldNotSetNonBlocking => "Could not set socket to non-blocking mode",
                _ => "Unknown error"
            }
        )
    }
}

impl Display for NetCodeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for NetCodeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl From<Error> for NetCodeErr {
    fn from(_: Error) -> Self {
        Self::UnknownError
    }
}