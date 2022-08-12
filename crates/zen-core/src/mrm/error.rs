use serde::de;
use std::{fmt, io};
use thiserror::Error;
use zen_parser::ascii;
use zen_parser::binary;

#[derive(Error, Debug)]
pub enum MrmError {
    #[error("{0}")]
    Message(String),
    #[error("MRM IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("MRM Binary Error: {0}")]
    Binary(#[from] binary::Error),
    #[error("MRM Ascii Error: {0}")]
    Ascii(#[from] ascii::Error),
    #[error("Expected Identifier: {0}")]
    ExpectedIdentifier(String),
    #[error("MRM Material Error: {0}")]
    Material(#[from] crate::material::MaterialError),
}

impl de::Error for MrmError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        MrmError::Message(msg.to_string())
    }
}

impl From<zen_parser::Error> for MrmError {
    fn from(e: zen_parser::Error) -> Self {
        match e {
            zen_parser::Error::Ascii(e) => Self::Ascii(e),
            zen_parser::Error::Binary(e) => Self::Binary(e),
            zen_parser::Error::Message(s) => Self::Message(s),
        }
    }
}

pub type MrmResult<T> = Result<T, MrmError>;
