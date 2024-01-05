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
use serde::Deserialize;
use std::{
    cell::RefCell,
    fmt,
    io::{Read, Seek, SeekFrom},
};
use zen_parser::prelude::*;

pub use entry::Entry;
pub use error::ArchiveError;

//pub mod bevy;
mod entry;
mod error;

const SIGNATURE_G1: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\r\n\r\n";
const SIGNATURE_G2: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\n\r\n\r";

const COMMENT_LENGTH: u64 = 256;
const SIGNATURE_LENGTH: usize = 16;
const ENTRY_NAME_LENGTH: usize = 64;
const ENTRY_DIR: u32 = 0x80000000;

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
    //deserializer: Arc<Mutex<BinaryDeserializer<R>>>,
    header: Header,
    obj: RefCell<BinaryDeserializer<R>>,
    //obj: RwLock<BinaryDeserializer<R>>,
    //pos: Cell<u64>,
}

impl<R: BinaryRead> fmt::Display for Vdfs<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VDFS Archive:\n\tSize: {}\n\tTime: {}\n\tOffset: {}\n\tNumber Files: {}",
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

        // deserializer
        //     .parser
        //     .seek(SeekFrom::Start(header.offset as u64))?;

        let obj = RefCell::new(deserializer);
        //let pos = Cell::new(header.offset as u64);

        Ok(Vdfs {
            //deserializer: Arc::new(Mutex::new(deserializer)),
            header,
            obj,
            //pos,
        })
    }

    /// Get all entries
    pub fn entries<'a>(&self) -> ArchiveResult<impl Iterator<Item = Entry<'_>> + '_> {
        // let mut deserializer = self
        //     .deserializer
        //     .lock()
        //     .map_err(|_| ArchiveError::PoisonError)?;

        let mut deserializer = self.obj.borrow_mut();
        deserializer.seek(SeekFrom::Start(self.header.offset as u64))?;

        Ok((0..self.header.count)
            .map(
                |index| -> ArchiveResult<(u32, String, u32, u32, u32, u32)> {
                    let mut deserializer = self.obj.borrow_mut();

                    let mut name_buf = [0_u8; ENTRY_NAME_LENGTH];
                    deserializer.read_exact(&mut name_buf)?;
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

                    Ok((index, name, offset, size, kind, attr))
                },
            )
            .filter_map(|res| match res {
                Ok((index, name, offset, size, kind, attr)) => {
                    let io = self.obj.borrow_mut();

                    if kind & ENTRY_DIR == 0 {
                        Some(Entry::new(
                            index,
                            name,
                            offset as u64,
                            size as u64,
                            kind,
                            attr,
                            io,
                        ))
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }))
    }
}
