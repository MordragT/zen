use crate::deserializer::Position;
use serde::de;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub code: ErrorCode,
    pub position: Position,
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error {
            code: ErrorCode::Message(msg.to_string()),
            position: Position { line: 0, column: 0 },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.position, self.code)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorCode {
    Message(String),
    InvalidDescriptor,
    Eof,
    UnknownValueKind,
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    ParseBoolError,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Message(s) => f.write_str(s),
            ErrorCode::InvalidDescriptor => f.write_str("Invalid Descriptor"),
            ErrorCode::Eof => f.write_str("Reached end of file"),
            ErrorCode::UnknownValueKind => f.write_str("Unknown Value Kind"),
            ErrorCode::ParseIntError(e) => fmt::Display::fmt(e, f),
            ErrorCode::ParseFloatError(e) => fmt::Display::fmt(e, f),
            ErrorCode::ParseBoolError => f.write_str("Error parsing Bool, value outside 1 or 0"),
        }
    }
}

impl From<ParseIntError> for ErrorCode {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ParseFloatError> for ErrorCode {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}
