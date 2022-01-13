use std::fmt;

#[derive(Debug)]
pub enum Error {
    LoadError(tobj::LoadError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadError(e) => f.write_str(&e.to_string()),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
