use super::Position;
use serde::de;
use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError, TryFromIntError};

pub type AsciiResult<T> = Result<T, AsciiError>;

/// [crate::AsciiDeserializer] Error
#[derive(Debug)]
pub struct AsciiError {
    pub code: AsciiErrorCode,
    pub position: Position,
}

impl std::error::Error for AsciiError {}

impl de::Error for AsciiError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        AsciiError {
            code: AsciiErrorCode::Message(msg.to_string()),
            position: Position { line: 0, column: 0 },
        }
    }
}

impl fmt::Display for AsciiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.position, self.code)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AsciiErrorCode {
    Io(String),
    Message(String),
    Expected(String),
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
}

impl fmt::Display for AsciiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AsciiErrorCode::Expected(s) => f.write_str(s),
            AsciiErrorCode::Io(s) => f.write_str(s),
            AsciiErrorCode::InvalidHeader => f.write_str("Invalid header"),
            AsciiErrorCode::ExpectedAsciiHeader => {
                f.write_str("Expeted Ascii header: object count")
            }
            AsciiErrorCode::ExpectedAsciiHeaderEnd => f.write_str("Expected Ascii header: END"),
            AsciiErrorCode::Message(s) => f.write_str(s),
            AsciiErrorCode::InvalidDescriptor => f.write_str("Invalid Descriptor"),
            AsciiErrorCode::EndOfFile => f.write_str("Reached end of file"),
            AsciiErrorCode::UnknownValueKind(s) => f.write_str(s),
            AsciiErrorCode::ParseIntError(e) => fmt::Display::fmt(e, f),
            AsciiErrorCode::ParseFloatError(e) => fmt::Display::fmt(e, f),
            AsciiErrorCode::ParseBoolError => {
                f.write_str("Error parsing Bool, value outside 1 or 0")
            }
            AsciiErrorCode::ParseColorError => {
                f.write_str("Error parsing Color, no (u8, u8, u8, u8)")
            }
            AsciiErrorCode::ParseBytesError => {
                f.write_str("Error parsing raw bytes, containing invalid digits")
            }
            AsciiErrorCode::ExpectedInt => f.write_str("Expected integer"),
            AsciiErrorCode::ExpectedFloat => f.write_str("Expected float"),
            AsciiErrorCode::ExpectedBool => f.write_str("Expected boolean"),
            AsciiErrorCode::ExpectedColor => f.write_str("Expected color"),
            AsciiErrorCode::ExpectedString => f.write_str("Expected string"),
            AsciiErrorCode::ExpectedBytes => f.write_str("Expected bytes"),
            AsciiErrorCode::ExpectedStructHeader => f.write_str("Expected struct header"),
            AsciiErrorCode::ExpectedStructVersion => f.write_str("Expected struct version"),
            AsciiErrorCode::ExpectedStructId => f.write_str("Expected struct id"),
            AsciiErrorCode::ExpectedStructEnd => f.write_str("Expected struct end"),
            AsciiErrorCode::InvalidStructHeader => f.write_str("Invalid struct header"),
            AsciiErrorCode::TryFromInt(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<ParseIntError> for AsciiErrorCode {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<ParseFloatError> for AsciiErrorCode {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}

impl From<TryFromIntError> for AsciiErrorCode {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromInt(e)
    }
}

impl From<io::Error> for AsciiError {
    fn from(e: io::Error) -> Self {
        AsciiError {
            code: AsciiErrorCode::Io(e.to_string()),
            position: Position { line: 0, column: 0 },
        }
    }
}
