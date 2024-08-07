use serde::de;
use std::{fmt, io};
use thiserror::Error;
use zen_parser::binary;

/// Error Object for Vdfs Archives
#[derive(Error, Debug)]
pub enum VdfsError {
    #[error("{0}")]
    Message(String),
    #[error("Archive IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Archive Binary Error: {0}")]
    Binary(#[from] binary::BinaryError),
    #[error("Unsupported version number in Vdfs header")]
    UnsupportedVersion,
    #[error("Unknown signature in Vdfs header")]
    UnknownSignature,
    #[error("Unknown entry type: {0}")]
    UnknownEntryKind(u32),
}

impl de::Error for VdfsError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        VdfsError::Message(msg.to_string())
    }
}

pub type VdfsResult<T> = Result<T, VdfsError>;
