use serde::de;
use std::{fmt, io};
use thiserror::Error;

/// [crate::BinaryDeserializer] Error
#[derive(Error, Debug)]
pub enum BinaryError {
    #[error("Message: {0}")]
    Message(String),
    #[error("BinaryIoError: {0}")]
    Io(#[from] io::Error),
    #[error("ExpectedAsciiChar: not {0}")]
    ExpectedAsciiChar(u8),
    #[error("ExpectedEndOfStr")]
    ExpectedEndOfStr,
    #[error("ExpectedBool")]
    ExpectedBool,
    #[error("UnexpectedEoF")]
    UnexpectedEoF,
    #[error("MissingBufferSize")]
    MissingBufferSize,
    #[error("InvalidHeader")]
    InvalidHeader,
    #[error("ExpectedSaveGame")]
    ExpectedSaveGame,
    #[error("UnexpectedByte")]
    UnexpectedByte,
}

impl de::Error for BinaryError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        BinaryError::Message(msg.to_string())
    }
}
pub type BinaryResult<T> = Result<T, BinaryError>;
