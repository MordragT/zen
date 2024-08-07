use serde::de;
use std::{fmt, io};

/// [crate::BinaryDeserializer] Error
#[derive(Debug)]
pub enum BinaryError {
    Message(String),
    Io(io::Error),
    ExpectedAsciiChar(u8),
    ExpectedBool,
}

impl fmt::Display for BinaryError {
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

impl std::error::Error for BinaryError {}

impl de::Error for BinaryError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        BinaryError::Message(msg.to_string())
    }
}

impl From<io::Error> for BinaryError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

pub type BinaryResult<T> = Result<T, BinaryError>;
