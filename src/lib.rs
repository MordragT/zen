#![feature(cell_update)]

use serde::Deserialize;
use std::fmt;
use std::io::BufRead;
use std::io::Read;
use zen_loader_from_reader::FromReader;

pub mod material;
pub mod math;
pub mod mesh;
pub mod texture;
pub mod vdfs;

pub enum FileKind {
    Tex = 538976288,
}

pub trait FromReader {
    fn from_reader<R: Read>(reader: R) -> Self;
}

pub trait FromBufReader {
    fn from_buf_reader<R: Read + BufRead>(reader: R) -> Self;
}

impl FromReader for u32 {
    fn from_reader<R: Read>(mut reader: R) -> Self {
        let mut buf = [0_u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buf).unwrap();
        bincode::deserialize(&buf).unwrap()
    }
}

impl FromReader for u16 {
    fn from_reader<R: Read>(mut reader: R) -> Self {
        let mut buf = [0_u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buf).unwrap();
        bincode::deserialize(&buf).unwrap()
    }
}

impl FromBufReader for String {
    fn from_buf_reader<R: Read + BufRead>(mut reader: R) -> Self {
        let mut buf = vec![];
        reader
            .read_until('\r' as u8 | '\n' as u8 | '\0' as u8, &mut buf)
            .unwrap();
        bincode::deserialize(&buf).unwrap()
    }
}

pub fn skip_spaces<R: Read>(mut reader: R) {
    loop {
        let mut buf = [0_u8; 1];
        match reader.read_exact(&mut buf) {
            Ok(_) => (),
            Err(_) => break,
        }
        let c = u8::from_le_bytes(buf) as char;
        match c {
            ' ' | '\r' | '\t' | '\n' => (),
            _ => break,
        }
    }
}

#[derive(FromReader, Deserialize, Debug)]
struct Date {
    year: u32,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}, {}:{}:{}",
            self.day, self.month, self.year, self.hour, self.minute, self.second
        )
    }
}

/// Possible filetypes this zen-file can have
pub enum Kind {
    Unknown,
    Ascii,
    Binary,
    BinSafe,
}
/// Information about one of the chunks in a zen-file
pub struct ChunkHeader {
    start_position: u32,
    size: u32,
    verison: u16,
    object_id: u32,
    name: String,
    class_name: String,
    create_object: bool,
}

#[derive(FromReader, Deserialize, Debug)]
pub struct Chunk {
    pub id: u16,
    pub length: u32,
}

/// File Header for zen-files
pub struct Header {
    version: i32,
    kind: Kind,
    save_game: bool,
    date: String,
    user: String,
    object_count: i32,
}

/// Header for BinSafe file kind
pub struct BinSafeHeader {
    version: u32,
    hash_table_offset: u32,
}
