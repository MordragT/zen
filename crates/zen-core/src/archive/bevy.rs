use super::{ArchiveError, Vdfs, SIGNATURE_G2};
use crate::{
    model::ZenModel,
    mrm::{Mrm, MrmError},
    msh::{Msh, MshError},
};
use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    prelude::{AddAsset, App, Plugin},
    reflect::{TypeUuid, Uuid},
};
use std::{io::Cursor, path::PathBuf};
use thiserror::Error;
use zen_parser::prelude::BinaryRead;

impl<R: BinaryRead> TypeUuid for Vdfs<R> {
    const TYPE_UUID: Uuid = Uuid::from_bytes(SIGNATURE_G2);
}

#[derive(Debug, Error)]
pub enum VdfsLoaderError {
    #[error("Vdfs Loader Archive Error: {0}")]
    Archive(#[from] ArchiveError),
    #[error("Vdfs Loader MRM Error: {0}")]
    Mrm(#[from] MrmError),
    #[error("Vdfs Loader MSH Error: {0}")]
    Msh(#[from] MshError),
}

/// An [AssetLoader] implementation for Vdfs archives.
/// The root path is the root of the archive.
pub struct VdfsLoader;

impl AssetLoader for VdfsLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let cursor = Cursor::new(bytes.to_vec());
            let vdfs = Vdfs::new(cursor)?;

            for entry in vdfs.entries() {
                match entry.name.as_bytes() {
                    [_, b'.', b'M', b'R', b'M'] => {
                        let bytes = vdfs.data(&entry)?;
                        let cursor = Cursor::new(bytes);
                        let asset = ZenModel::try_from(Mrm::new(cursor, entry.name)?)?;
                        load_context.set_labeled_asset(entry.name, LoadedAsset::new(asset));
                    }
                    [_, b'.', b'M', b'S', b'H'] => {
                        let bytes = vdfs.data(&entry)?;
                        let cursor = Cursor::new(bytes);
                        let asset = ZenModel::try_from(Msh::new(cursor, entry.name)?)?;
                        load_context.set_labeled_asset(entry.name, LoadedAsset::new(asset));
                    }
                    [_, b'.', b'T', b'G', b'A'] => {
                        let bytes = vdfs.data(&entry)?;
                        let cursor = Cursor::new(bytes);
                    }
                    _ => (),
                }
            }

            load_context.set_default_asset(LoadedAsset::new(vdfs));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vdfs", "VDFS"]
    }
}

pub struct VdfsPlugin(PathBuf);

impl VdfsPlugin {
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }
}

impl Plugin for VdfsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset_loader(VdfsLoader);
    }
}
