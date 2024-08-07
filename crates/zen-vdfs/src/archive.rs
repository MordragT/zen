use core::fmt;
use serde::Deserialize;
use std::{
    io::{self, SeekFrom},
    sync::Mutex,
};

use zen_parser::binary::{BinaryDeserializer, BinaryRead};

use crate::{
    entry::{VdfsEntries, VdfsEntry},
    error::VdfsResult,
    header::VdfsHeader,
};

/// Vdfs archive reader
#[derive(Debug)]
pub struct VdfsArchive<H> {
    handle: Mutex<H>,
    header: VdfsHeader,
    entries: VdfsEntries,
}

impl<H> VdfsArchive<H> {
    pub fn len(&self) -> usize {
        self.header.count as usize
    }

    pub fn size(&self) -> u32 {
        self.header.data_size
    }

    pub fn timestamp(&self) -> u32 {
        self.header.timestamp
    }

    pub fn entries(&self) -> impl Iterator<Item = &VdfsEntry> {
        self.entries.values()
    }

    pub fn get(&self, k: impl AsRef<str>) -> Option<VdfsEntry> {
        self.entries.get(k.as_ref()).cloned()
    }
}

impl<H: BinaryRead> VdfsArchive<H> {
    const COMMENT_LENGTH: u64 = 256;

    /// Creates a new Vdfs struct that holds the data of all entries
    pub fn new(mut handle: H) -> VdfsResult<Self> {
        handle.seek(SeekFrom::Start(Self::COMMENT_LENGTH))?;

        let mut deser = BinaryDeserializer::from(&mut handle);
        let header = VdfsHeader::deserialize(&mut deser)?;
        header.validate()?;

        let entries = header.read_entries(&mut handle)?;
        let handle = Mutex::new(handle);

        Ok(Self {
            handle,
            header,
            entries,
        })
    }

    pub fn fetch(&self, entry: &VdfsEntry) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; entry.size as usize];

        let mut guard = self.handle.lock().unwrap();
        guard.seek(SeekFrom::Start(entry.offset as u64))?;
        guard.read_exact(&mut buf)?;

        Ok(buf)
    }

    pub fn fetch_mut(&mut self, entry: &VdfsEntry) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; entry.size as usize];

        let handle = self.handle.get_mut().unwrap();
        handle.seek(SeekFrom::Start(entry.offset as u64))?;
        handle.read_exact(&mut buf)?;

        Ok(buf)
    }
}

impl<H> fmt::Display for VdfsArchive<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.entries.values() {
            write!(f, "{entry}\n")?;
        }

        self.header.fmt(f)
    }
}

#[cfg(feature = "bevy")]
mod bevy {
    use bevy::{
        asset::io::{AssetReader, AssetReaderError, PathStream, Reader},
        tasks::futures_lite::io::Cursor,
        utils::ConditionalSendFuture,
    };
    use std::path::Path;
    use zen_parser::binary::BinaryRead;

    use super::VdfsArchive;

    impl<H: BinaryRead + Send + Sync + 'static> AssetReader for VdfsArchive<H> {
        fn read<'a>(
            &'a self,
            path: &'a Path,
        ) -> impl ConditionalSendFuture<Output = Result<Box<Reader<'a>>, AssetReaderError>>
        {
            Box::pin(async move {
                let entry = self
                    .get(path.to_string_lossy())
                    .ok_or(AssetReaderError::NotFound(path.to_owned()))?;
                let data = self.fetch(&entry)?;

                Ok(Box::new(Cursor::new(data)) as Box<Reader<'a>>)
            })
        }

        fn read_meta<'a>(
            &'a self,
            _path: &'a Path,
        ) -> impl ConditionalSendFuture<Output = Result<Box<Reader<'a>>, AssetReaderError>>
        {
            Box::pin(async { unimplemented!() })
        }

        fn read_directory<'a>(
            &'a self,
            _path: &'a Path,
        ) -> impl ConditionalSendFuture<Output = Result<Box<PathStream>, AssetReaderError>>
        {
            Box::pin(async { unimplemented!() })
        }

        fn is_directory<'a>(
            &'a self,
            _path: &'a Path,
        ) -> impl ConditionalSendFuture<Output = Result<bool, AssetReaderError>> {
            Box::pin(async { Ok(false) })
        }
    }
}
