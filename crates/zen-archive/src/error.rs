use serde::de;
use std::{fmt, io};
use zen_parser::binary;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),
    Binary(binary::Error),
    UnsupportedVersion,
    UnknownSignature,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(s) => f.write_str(&s),
            Self::Io(e) => f.write_str(&e.to_string()),
            Self::Binary(e) => f.write_str(&e.to_string()),
            Self::UnsupportedVersion => f.write_str("Unsupported version number in Vdfs header"),
            Self::UnknownSignature => f.write_str("Unknown signature in Vdfs header"),
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

impl From<binary::Error> for Error {
    fn from(e: binary::Error) -> Self {
        Self::Binary(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
