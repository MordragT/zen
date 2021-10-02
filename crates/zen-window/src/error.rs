use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, WindowError>;

#[derive(Debug)]
pub enum WindowError {}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str("default"),
        }
    }
}

impl Error for WindowError {}
