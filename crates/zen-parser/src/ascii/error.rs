use super::Position;
use serde::de;
use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError, TryFromIntError};

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
    Io(String),
    Message(String),
    InvalidHeader,
    ExpectedAsciiHeader,
    ExpectedAsciiHeaderEnd,
    InvalidDescriptor,
    EndOfFile,
    UnknownValueKind(String),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    ParseBoolError,
    ParseColorError,
    ParseBytesError,
    ExpectedInt,
    ExpectedFloat,
    ExpectedBool,
    ExpectedColor,
    ExpectedString,
    ExpectedBytes,
    ExpectedStructHeader,
    ExpectedStructVersion,
    ExpectedStructId,
    ExpectedStructEnd,
    InvalidStructHeader,
    TryFromInt(TryFromIntError),
    DeserializeNotSupported(String),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Io(s) => f.write_str(s),
            ErrorCode::InvalidHeader => f.write_str("Invalid header"),
            ErrorCode::ExpectedAsciiHeader => f.write_str("Expeted Ascii header: object count"),
            ErrorCode::ExpectedAsciiHeaderEnd => f.write_str("Expected Ascii header: END"),
            ErrorCode::Message(s) => f.write_str(s),
            ErrorCode::InvalidDescriptor => f.write_str("Invalid Descriptor"),
            ErrorCode::EndOfFile => f.write_str("Reached end of file"),
            ErrorCode::UnknownValueKind(s) => f.write_str(s),
            ErrorCode::ParseIntError(e) => fmt::Display::fmt(e, f),
            ErrorCode::ParseFloatError(e) => fmt::Display::fmt(e, f),
            ErrorCode::ParseBoolError => f.write_str("Error parsing Bool, value outside 1 or 0"),
            ErrorCode::ParseColorError => f.write_str("Error parsing Color, no (u8, u8, u8, u8)"),
            ErrorCode::ParseBytesError => {
                f.write_str("Error parsing raw bytes, containing invalid digits")
            }
            ErrorCode::ExpectedInt => f.write_str("Expected integer"),
            ErrorCode::ExpectedFloat => f.write_str("Expected float"),
            ErrorCode::ExpectedBool => f.write_str("Expected boolean"),
            ErrorCode::ExpectedColor => f.write_str("Expected color"),
            ErrorCode::ExpectedString => f.write_str("Expected string"),
            ErrorCode::ExpectedBytes => f.write_str("Expected bytes"),
            ErrorCode::ExpectedStructHeader => f.write_str("Expected struct header"),
            ErrorCode::ExpectedStructVersion => f.write_str("Expected struct version"),
            ErrorCode::ExpectedStructId => f.write_str("Expected struct id"),
            ErrorCode::ExpectedStructEnd => f.write_str("Expected struct end"),
            ErrorCode::InvalidStructHeader => f.write_str("Invalid struct header"),
            ErrorCode::TryFromInt(e) => fmt::Display::fmt(e, f),
            ErrorCode::DeserializeNotSupported(s) => {
                f.write_str(format!("Deserialize not supported: {}", s).as_str())
            }
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

impl From<TryFromIntError> for ErrorCode {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromInt(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            code: ErrorCode::Io(e.to_string()),
            position: Position { line: 0, column: 0 },
        }
    }
}
