use bevy::{
    asset::io::{AssetReader, AssetReaderError, PathStream, Reader},
    tasks::futures_lite::io::Cursor,
};
use std::path::Path;
use zen_parser::binary::BinaryRead;

use crate::VdfsArchive;

impl<H: BinaryRead + Send + Sync + 'static> AssetReader for VdfsArchive<H> {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<Box<Reader<'a>>, AssetReaderError> {
        let entry = self
            .get(path.to_string_lossy())
            .ok_or(AssetReaderError::NotFound(path.to_owned()))?;
        let data = self.fetch(&entry)?;

        Ok(Box::new(Cursor::new(data)) as Box<Reader<'a>>)
    }

    async fn read_meta<'a>(&'a self, _path: &'a Path) -> Result<Box<Reader<'a>>, AssetReaderError> {
        unimplemented!()
    }

    async fn read_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        unimplemented!()
    }

    async fn is_directory<'a>(&'a self, _path: &'a Path) -> Result<bool, AssetReaderError> {
        Ok(false)
    }
}
