use serde::de;
use std::{fmt, io};

/// [crate::BinaryDeserializer] Error
#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),
    ExpectedAsciiChar(u8),
    ExpectedBool,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(s) => f.write_str(&s),
            Self::Io(e) => f.write_str(&e.to_string()),
            Self::ExpectedAsciiChar(b) => {
                f.write_str(&format!("Expected ascii char, not: '{}'", b))
            }
            Self::ExpectedBool => f.write_str("Expected bool"),
        }
    }
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
