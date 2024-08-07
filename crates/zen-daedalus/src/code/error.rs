use std::{fmt, io};

/// The Error object for the [code](crate::code)
#[derive(Debug)]
pub enum Error {
    Binary(zen_parser::binary::BinaryError),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Binary(e) => f.write_str(&e.to_string()),
            Self::Io(e) => f.write_str(&e.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl From<zen_parser::binary::BinaryError> for Error {
    fn from(e: zen_parser::binary::BinaryError) -> Self {
        Error::Binary(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
