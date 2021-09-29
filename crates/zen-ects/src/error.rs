use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, EctsError>;

#[derive(Debug)]
pub enum EctsError {
    OutOfBounds(usize),
    EventNotFound(usize),
}

impl fmt::Display for EctsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds(i) => f.write_str(&format!("entity out of bounds: {}", i)),
            Self::EventNotFound(e) => f.write_str(&format!("event not found: {}", e)),
        }
    }
}

impl Error for EctsError {}
