use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZTexError {
    #[error("Wrong ZTEX signature")]
    WrongSignature,
    #[error("Wrong ZTEX version")]
    WrongVersion,
    #[error("ZTEX Binary Error: {0}")]
    Binary(#[from] zen_parser::binary::BinaryError),
    #[error("ZTEX IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type ZTexResult<T> = Result<T, ZTexError>;
