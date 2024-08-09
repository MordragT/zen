use image::ImageError;
use serde::de;
use std::{fmt, num::ParseIntError};
use thiserror::Error;
use zen_parser::binary;

use crate::texture::error::ZTexError;

#[derive(Error, Debug)]
pub enum ZMatError {
    #[error("Unknown version: {0}")]
    UnknownVersion(u16),
    #[error("{0}")]
    Message(String),
    #[error(transparent)]
    Binary(#[from] binary::BinaryError),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    ZTex(#[from] ZTexError),
    #[error(transparent)]
    Image(#[from] ImageError),
}

impl de::Error for ZMatError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ZMatError::Message(msg.to_string())
    }
}

pub type ZMatResult<T> = Result<T, ZMatError>;
