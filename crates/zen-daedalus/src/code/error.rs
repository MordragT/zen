use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Binary(zen_parser::binary::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Binary(e) => f.write_str(&e.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl From<zen_parser::binary::Error> for Error {
    fn from(e: zen_parser::binary::Error) -> Self {
        Error::Binary(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
