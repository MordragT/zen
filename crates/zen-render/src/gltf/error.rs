use bevy::asset::ReadAssetBytesError;
use image::ImageError;
use thiserror::Error;

use crate::texture::error::ZTexError;

#[derive(Debug, Error)]
pub enum GltfError {
    #[error(transparent)]
    Texture(#[from] ZTexError),
    #[error(transparent)]
    Image(#[from] ImageError),
    #[error(transparent)]
    ReadAsset(#[from] ReadAssetBytesError),
}

pub type GltfResult<T> = Result<T, GltfError>;
