use core::fmt;
use std::{io, sync::Mutex};

use zen_parser::binary::{BinaryDecoder, BinaryIoReader, BinaryRead, BinarySliceReader};

use crate::{
    entry::{VdfsEntries, VdfsEntry},
    error::VdfsResult,
    header::VdfsHeader,
};

/// Vdfs archive reader
#[derive(Debug)]
pub struct VdfsArchive<R> {
    decoder: Mutex<BinaryDecoder<R>>,
    header: VdfsHeader,
    entries: VdfsEntries,
}

impl<R> VdfsArchive<R> {
    const COMMENT_LENGTH: u64 = 256;

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

impl<R> VdfsArchive<BinaryIoReader<R>>
where
    R: io::BufRead + io::Seek,
{
    pub fn from_reader(reader: R) -> VdfsResult<Self> {
        let decoder = BinaryDecoder::from_reader(reader);
        Self::from_decoder(decoder)
    }
}

impl<'a> VdfsArchive<BinarySliceReader<'a>> {
    pub fn from_slice(slice: &'a [u8]) -> VdfsResult<Self> {
        let decoder = BinaryDecoder::from_slice(slice);
        Self::from_decoder(decoder)
    }
}

impl<R> VdfsArchive<R>
where
    R: BinaryRead,
{
    // /// Creates a new Vdfs struct that holds the data of all entries
    pub fn from_decoder(mut decoder: BinaryDecoder<R>) -> VdfsResult<Self> {
        decoder.set_position(Self::COMMENT_LENGTH)?;

        let header = decoder.decode::<VdfsHeader>()?;
        header.validate()?;

        let entries = header.read_entries(&mut decoder)?;
        let decoder = Mutex::new(decoder);

        Ok(Self {
            decoder,
            header,
            entries,
        })
    }

    pub fn fetch(&self, entry: &VdfsEntry) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; entry.size as usize];

        let mut guard = self.decoder.lock().unwrap();
        guard.set_position(entry.offset as u64)?;
        guard.read_bytes(&mut buf)?;

        Ok(buf)
    }

    pub fn fetch_mut(&mut self, entry: &VdfsEntry) -> io::Result<Vec<u8>> {
        let mut buf = vec![0; entry.size as usize];

        let decoder = self.decoder.get_mut().unwrap();
        decoder.set_position(entry.offset as u64)?;
        decoder.read_bytes(&mut buf)?;

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
