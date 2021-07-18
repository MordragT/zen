use std::{fmt, io};
use zen_parser::ascii;
use zen_parser::binary;

pub type Result<T> = std::result::Result<T, Error>;

/// Error Type
#[derive(Debug)]
pub enum Error {
    WrongSignature,
    Conversion,
    Binary(binary::Error),
    Ascii(ascii::Error),
    Io(io::Error),
    #[cfg(feature = "image")]
    Image(image::ImageError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongSignature => f.write_str("Wrong ZTEX Signature or Version"),
            Self::Conversion => f.write_str("Couldnt convert ZTEX format to DDS format."),
            Self::Binary(e) => f.write_str(&e.to_string()),
            Self::Ascii(e) => f.write_str(&e.to_string()),
            Self::Io(e) => f.write_str(&e.to_string()),
            #[cfg(feature = "image")]
            Self::Image(e) => f.write_str(&e.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl From<binary::Error> for Error {
    fn from(e: binary::Error) -> Self {
        Self::Binary(e)
    }
}

impl From<ascii::Error> for Error {
    fn from(e: ascii::Error) -> Self {
        Self::Ascii(e)
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
#[cfg(feature = "image")]
impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Self::Image(e)
    }
}
