use crate::{ascii, binary};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

/// Error Type for all Deserializers
#[derive(Debug)]
pub enum Error {
    Binary(binary::Error),
    Ascii(ascii::Error),
    Message(String),
}

impl From<binary::Error> for Error {
    fn from(e: binary::Error) -> Self {
        Error::Binary(e)
    }
}

impl From<ascii::Error> for Error {
    fn from(e: ascii::Error) -> Self {
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
