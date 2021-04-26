use serde::de;
use std::{fmt, io};

/// Error Object for Vdfs Archives
#[derive(Debug)]
pub enum Error {
    Message(String),
    Io(io::Error),
    ZTex(zen_texture::Error),
    ExpectedValidTextureName(String),
    ZenArchive(zen_archive::Error),
    WrongTextureNameFormat,
    DDSFile(zen_texture::ddsfile::Error),
    Image(image::ImageError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(s) => f.write_str(&s),
            Self::Io(e) => f.write_str(&e.to_string()),
            Self::ExpectedValidTextureName(s) => f.write_str(&s),
            Self::ZTex(e) => f.write_str(&e.to_string()),
            Self::ZenArchive(e) => f.write_str(&e.to_string()),
            Self::WrongTextureNameFormat => f.write_str("Wrong texture name format!"),
            Self::DDSFile(e) => f.write_str(&e.to_string()),
            Self::Image(e) => f.write_str(&e.to_string()),
        }
    }
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<zen_texture::Error> for Error {
    fn from(e: zen_texture::Error) -> Self {
        Self::ZTex(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<zen_archive::Error> for Error {
    fn from(e: zen_archive::Error) -> Self {
        Self::ZenArchive(e)
    }
}

impl From<zen_texture::ddsfile::Error> for Error {
    fn from(e: zen_texture::ddsfile::Error) -> Self {
        Self::DDSFile(e)
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Self::Image(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
