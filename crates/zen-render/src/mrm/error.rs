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
    Binary(#[from] binary::BinaryError),
    #[error("MRM Ascii Error: {0}")]
    Ascii(#[from] ascii::AsciiError),
    #[error("Expected Identifier: {0}")]
    ExpectedIdentifier(String),
    #[error("Unexpected header id: {0}")]
    UnexpectedHeaderId(u16),
}

impl de::Error for MrmError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        MrmError::Message(msg.to_string())
    }
}

pub type MrmResult<T> = Result<T, MrmError>;
