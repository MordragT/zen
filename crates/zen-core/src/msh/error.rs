use serde::de;
use std::{fmt, io};
use thiserror::Error;
use zen_parser::ascii;
use zen_parser::binary;

#[derive(Error, Debug)]
pub enum MshError {
    #[error("{0}")]
    Message(String),
    #[error("MSH IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("MSH Binary Error: {0}")]
    Binary(#[from] binary::Error),
    #[error("MSH Ascii Error: {0}")]
    Ascii(#[from] ascii::Error),
    #[error("Unknown game version {0}, please patch")]
    UnknownGameVersion(u32),
    #[error("Expected value: {0}")]
    ExpectedValue(String),
}

impl de::Error for MshError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        MshError::Message(msg.to_string())
    }
}

impl From<zen_parser::Error> for MshError {
    fn from(e: zen_parser::Error) -> Self {
        match e {
            zen_parser::Error::Ascii(e) => Self::Ascii(e),
            zen_parser::Error::Binary(e) => Self::Binary(e),
            zen_parser::Error::Message(s) => Self::Message(s),
        }
    }
}

pub type MshResult<T> = Result<T, MshError>;
