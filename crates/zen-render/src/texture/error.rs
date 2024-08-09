use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZTexError {
    #[error("Wrong ZTEX signature")]
    WrongSignature,
    #[error("Wrong ZTEX version")]
    WrongVersion,
    #[error(transparent)]
    Binary(#[from] zen_parser::binary::BinaryError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
}

pub type ZTexResult<T> = Result<T, ZTexError>;
