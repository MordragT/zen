use crate::{ascii, binary};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

/// Error Type for all Deserializers
#[derive(Debug)]
pub enum Error {
    Binary(binary::BinaryError),
    Ascii(ascii::AsciiError),
    Message(String),
}

impl From<binary::BinaryError> for Error {
    fn from(e: binary::BinaryError) -> Self {
        Error::Binary(e)
    }
}

impl From<ascii::AsciiError> for Error {
    fn from(e: ascii::AsciiError) -> Self {
        Error::Ascii(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ascii(e) => f.write_str(&e.to_string()),
            Self::Binary(e) => f.write_str(&e.to_string()),
            Self::Message(s) => f.write_str(&s),
        }
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
