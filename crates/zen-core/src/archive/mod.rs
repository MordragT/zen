//! Crate to open Vdfs Files and access the different entries.
//!
//! The given example loads a wave audio file out of an archive.
//! ```rust
//! use std::{fs::File, io::Write};
//! use zen_archive::Vdfs;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Sounds.vdf")?;
//! let vdf = Vdfs::new(vdf_file)?;
//! let entry = vdf
//!     .get_by_name_slice("CHAPTER_01.WAV")
//!     .expect("Should be there!");
//! let mut audio_file = File::create("/home/tom/Git/zen-loader/files/audio/chapter_01.wav")?;
//! audio_file.write(&entry.data)?;
//! # Ok(())
//! # }
//! ```

use error::ArchiveResult;
use serde::{Deserialize, __private::de::IdentifierDeserializer};
use std::{
    fmt, io,
    io::{prelude::*, Take},
    io::{Read, SeekFrom},
    sync::{Arc, Mutex},
};
use zen_parser::prelude::*;

pub use error::ArchiveError;

//pub mod bevy;
mod error;

const SIGNATURE_G1: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\r\n\r\n";
const SIGNATURE_G2: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\n\r\n\r";

const COMMENT_LENGTH: u64 = 256;
const SIGNATURE_LENGTH: usize = 16;
const ENTRY_NAME_LENGTH: usize = 64;
const ENTRY_DIR: u32 = 0x80000000;

/// Vdfs Entry
pub struct Entry<R: BinaryRead> {
    name: String,
    cursor: u64,
    offset: u64,
    size: u64,
    kind: u32,
    attr: u32,
    deserializer: Arc<Mutex<BinaryDeserializer<R>>>,
}

impl<R: BinaryRead> Entry<R> {
    fn new(
        name: String,
        offset: u64,
        size: u64,
        kind: u32,
        attr: u32,
        deserializer: Arc<Mutex<BinaryDeserializer<R>>>,
    ) -> Self {
        Self {
            name,
            cursor: offset,
            offset,
            size,
            kind,
            attr,
            deserializer,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl<'a, R: BinaryRead> fmt::Display for Entry<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}, Offset: {:x}, Size: {:x}, Kind: {:x}, Attr: {:x}",
            self.name, self.offset, self.size, self.kind, self.attr
        )
    }
}

impl<R: BinaryRead> Read for Entry<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.cursor - self.offset >= self.size {
            return Ok(0);
        }

        let mut deserializer = self
            .deserializer
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, ArchiveError::PoisonError))?;

        // TODO should not have to be called everytime
        deserializer.seek(SeekFrom::Start(self.cursor))?;

        let len = buf.len();
        if len <= self.size as usize {
            deserializer.read_exact(buf)?;
            self.cursor += len as u64;
            Ok(len)
        } else {
            // TODO rewrite when Take implements Seek

            let mut exact_buf = vec![0; self.size as usize];

            deserializer.read_exact(&mut exact_buf)?;
            self.cursor += self.size;

            buf.copy_from_slice(&exact_buf);

            Ok(self.size as usize)
        }
    }
}

impl<R: BinaryRead> Seek for Entry<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let mut deserializer = self
            .deserializer
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, ArchiveError::PoisonError))?;

        match pos {
            SeekFrom::Current(pos) => {
                let new_cursor = (self.cursor as i64 + pos) as u64;

                if new_cursor < self.offset {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Tried to seek to negative offset",
                    ))
                } else {
                    deserializer.seek(SeekFrom::Start(new_cursor))?;
                    self.cursor = new_cursor;
                    Ok(self.cursor - self.offset)
                }
            }
            SeekFrom::Start(pos) => {
                // Ok to seek above end of Reader??!!
                let new_cursor = self.cursor + pos;
                let res = deserializer.seek(SeekFrom::Start(new_cursor))?;
                self.cursor = new_cursor;
                Ok(res)
            }
            SeekFrom::End(pos) => {
                let new_cursor = ((self.offset + self.size) as i64 + pos) as u64;

                if new_cursor < self.offset {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Tried to seek to negative offset",
                    ))
                } else {
                    deserializer.seek(SeekFrom::Start(new_cursor))?;
                    self.cursor = new_cursor;
                    Ok(self.cursor - self.offset)
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
/// Vdfs Header attributes
struct Header {
    signature: [u8; SIGNATURE_LENGTH],
    count: u32,
    num_files: u32,
    timestamp: u32,
    data_size: u32,
    offset: u32,
    version: u32,
}

pub enum VdfsKind {
    Animation,
    Mesh,
    Sound,
    Texture,
    World,
}

/// Vdfs archive reader
pub struct Vdfs<R: BinaryRead> {
    deserializer: Arc<Mutex<BinaryDeserializer<R>>>,
    header: Header,
}

impl<R: BinaryRead> fmt::Display for Vdfs<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VDFS Archive, Size: {:x}, Time: {:x}, Offset: {:x}, Number Files: {}",
            self.header.data_size, self.header.timestamp, self.header.offset, self.header.num_files
        )?;

        Ok(())
    }
}

impl<R: BinaryRead> Vdfs<R> {
    /// Creates a new Vdfs struct that holds the data of all entries
    pub fn new<'a>(reader: R) -> ArchiveResult<Vdfs<R>> {
        let mut deserializer = BinaryDeserializer::from(reader);
        deserializer.seek(SeekFrom::Start(COMMENT_LENGTH))?;
        let header = Header::deserialize(&mut deserializer)?;
        if header.signature != SIGNATURE_G1 && header.signature != SIGNATURE_G2 {
            return Err(ArchiveError::UnknownSignature);
        }

        if header.version != 0x50 {
            return Err(ArchiveError::UnsupportedVersion);
        }

        deserializer
            .parser
            .seek(SeekFrom::Start(header.offset as u64))?;

        Ok(Vdfs {
            deserializer: Arc::new(Mutex::new(deserializer)),
            header,
        })
    }

    /// Get all entries
    pub fn entries(&self) -> ArchiveResult<impl Iterator<Item = Entry<R>> + '_> {
        let mut deserializer = self
            .deserializer
            .lock()
            .map_err(|_| ArchiveError::PoisonError)?;

        Ok((0..self.header.count)
            .map(move |_| -> ArchiveResult<(String, u32, u32, u32, u32)> {
                let mut name_buf = [0_u8; ENTRY_NAME_LENGTH];
                deserializer.parser.read_exact(&mut name_buf)?;
                let name = name_buf
                    .iter()
                    .filter_map(|c| {
                        if *c >= 'A' as u8 && *c <= 'Z' as u8
                            || *c == '_' as u8
                            || *c == '.' as u8
                            || *c == '-' as u8
                            || *c >= '0' as u8 && *c <= '9' as u8
                        {
                            Some(*c as char)
                        } else {
                            None
                        }
                    })
                    .collect::<String>();

                let offset = u32::deserialize(deserializer.by_ref())?;
                let size = u32::deserialize(deserializer.by_ref())?;
                let kind = u32::deserialize(deserializer.by_ref())?;
                let attr = u32::deserialize(deserializer.by_ref())?;

                Ok((name, offset, size, kind, attr))
            })
            .filter_map(|res| match res {
                Ok((name, offset, size, kind, attr)) => {
                    if kind & ENTRY_DIR == 0 {
                        let deserializer = self.deserializer.clone();

                        Some(Entry::new(
                            name,
                            offset as u64,
                            size as u64,
                            kind,
                            attr,
                            deserializer,
                        ))
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }))
    }
}
