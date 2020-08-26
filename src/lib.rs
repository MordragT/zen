#![feature(cell_update)]

use serde::Deserialize;
use std::fmt;
use std::io::{BufRead, Read};
use zen_loader_from_reader::FromReader;

pub mod deserializer;
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

impl FromReader for u8 {
    fn from_reader<R: Read>(mut reader: R) -> Self {
        let mut buf = [0_u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buf).unwrap();
        bincode::deserialize(&buf).unwrap()
    }
}

impl FromReader for char {
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

pub fn skip_string<R: Read>(mut reader: R, pattern: &str) -> bool {
    skip_spaces(&mut reader);
    match pattern.is_empty() {
        true => loop {
            let c = char::from_reader(reader);
            if c == '\n' || c == ' ' {
                return true;
            }
        },
        false => {
            for pattern_char in pattern.chars() {
                let c = char::from_reader(reader);
                if c != pattern_char {
                    return false;
                }
            }
            true
        }
    }
}

pub fn read_i32_ascii<'a, R: Read>(mut reader: R) -> Result<i32, &'a str> {
    skip_spaces(reader);
    let num = String::new();
    loop {
        let digit = u8::from_reader(reader);
        if digit <= '0' as u8 || digit >= '9' as u8 {
            break;
        }
        num.push(digit as char);
    }
    match num.is_empty() {
        true => Err("No number found."),
        false => Ok(i32::from_str_radix(num.as_str(), 10).unwrap()),
    }
}

pub fn read_bool_ascii<'a, R: Read>(mut reader: R) -> Result<bool, &'a str> {
    skip_spaces(reader);
    let val = char::from_reader(reader);
    match val {
        '0' => Ok(false),
        '1' => Ok(true),
        _ => Err("Value is not a bool."),
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
