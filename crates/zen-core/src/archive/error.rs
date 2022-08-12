use serde::de;
use std::{fmt, io};
use thiserror::Error;
use zen_parser::binary;

/// Error Object for Vdfs Archives
#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("{0}")]
    Message(String),
    #[error("Archive IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Archive Binary Error: {0}")]
    Binary(#[from] binary::Error),
    #[error("Unsupported version number in Vdfs header")]
    UnsupportedVersion,
    #[error("Unknown signature in Vdfs header")]
    UnknownSignature,
    #[error("Poison error")]
    PoisonError,
}

impl de::Error for ArchiveError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ArchiveError::Message(msg.to_string())
    }
}

pub type ArchiveResult<T> = Result<T, ArchiveError>;
