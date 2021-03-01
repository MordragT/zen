use serde::de;
use std::{fmt, io};
use zen_parser::ascii;
use zen_parser::binary;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),
    Binary(binary::Error),
    Ascii(ascii::Error),
    UnknownGameVersion,
    ExpectedIdentifier(String),
    ExpectedValue(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(s) => f.write_str(&s),
            Self::Io(e) => f.write_str(&e.to_string()),
            Self::Binary(e) => f.write_str(&e.to_string()),
            Self::Ascii(e) => f.write_str(&e.to_string()),
            Self::UnknownGameVersion => f.write_str("Unknown game version, please patch"),
            Self::ExpectedIdentifier(s) => f.write_str(&s),
            Self::ExpectedValue(s) => f.write_str(&s),
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

impl From<ascii::Error> for Error {
    fn from(e: ascii::Error) -> Self {
        Self::Ascii(e)
    }
}

impl From<zen_parser::Error> for Error {
    fn from(e: zen_parser::Error) -> Self {
        match e {
            zen_parser::Error::Ascii(e) => Self::Ascii(e),
            zen_parser::Error::Binary(e) => Self::Binary(e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
