use std::{fmt, io};
use thiserror::Error;
use zen_parser::ascii;
use zen_parser::binary;

/// Error Type
#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Wrong ZTEX Signature or Version")]
    WrongSignature,
    #[error("Couldnt convert ZTEX format to DDS format.")]
    Conversion,
    #[error("Texture Binary Error: {0}")]
    Binary(#[from] binary::Error),
    #[error("Texture Ascii Error: {0}")]
    Ascii(#[from] ascii::Error),
    #[error("Texture IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Texture Image Error: {0}")]
    Image(#[from] image::ImageError),
}

pub type TextureResult<T> = Result<T, TextureError>;
