use serde::de;
use std::{fmt, io};
use thiserror::Error;

/// Error Object for Vdfs Archives
#[derive(Error, Debug)]
pub enum MaterialError {
    #[error("{0}")]
    Message(String),
    #[error("Material IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Material Texture Error: {0}")]
    ZTex(#[from] crate::texture::TextureError),
    #[error("Expected valid texture name: {0}")]
    ExpectedValidTextureName(String),
    #[error("Material Archive Error: {0}")]
    ZenArchive(#[from] crate::archive::ArchiveError),
    #[error("Wrong texture name format!")]
    WrongTextureNameFormat,
}
impl de::Error for MaterialError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        MaterialError::Message(msg.to_string())
    }
}

pub type MaterialResult<T> = Result<T, MaterialError>;
