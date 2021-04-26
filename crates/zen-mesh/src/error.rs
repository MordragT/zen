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
    UnknownGameVersion(u32),
    ExpectedIdentifier(String),
    ExpectedValue(String),
    Material(zen_material::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(s) => f.write_str(&s),
            Self::Io(e) => f.write_str(&e.to_string()),
            Self::Binary(e) => f.write_str(&e.to_string()),
            Self::Ascii(e) => f.write_str(&e.to_string()),
            Self::UnknownGameVersion(version) => {
                f.write_str(&format!("Unknown game version {}, please patch", version))
            }
            Self::ExpectedIdentifier(s) => f.write_str(&s),
            Self::ExpectedValue(s) => f.write_str(&s),
            Self::Material(e) => f.write_str(&e.to_string()),
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
            zen_parser::Error::Message(s) => Self::Message(s),
        }
    }
}

impl From<zen_material::error::Error> for Error {
    fn from(e: zen_material::error::Error) -> Self {
        Self::Material(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
