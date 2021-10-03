use hecs::ComponentError;
use std::io::Error as IoError;
use std::{fmt, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AssetLoaderError(Box<dyn std::error::Error>),
    ExpectedFile(PathBuf),
    ExpectedFileExtension,
    WrongFileExtension(String, String),
    AssetLoaderNotFound(String),
    ComponentError(ComponentError),
    IoError(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AssetLoaderError(e) => f.write_str(&e.to_string()),
            Self::ExpectedFile(p) => f.write_str(&format!("Expected file at: {:?}", p.as_os_str())),
            Self::ExpectedFileExtension => f.write_str("Expected a file extension"),
            Self::WrongFileExtension(exp, got) => f.write_str(&format!(
                "Wrong file extension: Expected: {}, Got: {}",
                exp, got
            )),
            Self::AssetLoaderNotFound(ext) => {
                f.write_str(&format!("Asset loader not found for extension: {}", ext))
            }
            Self::ComponentError(e) => f.write_str(&e.to_string()),
            Self::IoError(e) => f.write_str(&e.to_string()),
        }
    }
}

impl From<ComponentError> for Error {
    fn from(e: ComponentError) -> Self {
        Self::ComponentError(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IoError(e)
    }
}

impl std::error::Error for Error {}
