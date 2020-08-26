use crate::FromBufReader;
use std::fmt;
use std::io::{BufRead, Read};

pub mod ascii;
pub mod bin_safe;

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Possible filetypes this zen-file can have
pub enum Kind {
    Unknown,
    Ascii,
    Binary,
    BinSafe,
}

/// Possible value-types
#[repr(i32)]
pub enum Value {
    Zero,                // 0x0
    String(String),      // 0x1
    Int(i32),            // 0x2
    Float(f32),          // 0x3
    Byte(u8),            // 0x4
    Word(u16),           // 0x5
    Bool(bool),          // 0x6
    Vec3(f32, f32, f32), // 0x7
    Color,               // 0x8
    Raw(Vec<u8>),        // 0x9
    Undef1,              // 0xA
    Undef2,              // 0xB
    Undef3,              // 0xC
    Undef4,              // 0xD
    Undef5,              // 0xE
    Undef6,              // 0xF
    RawFloat,            // 0x10
    Enum,                // 0x11
    Hash,                // 0x12
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

impl Header {
    fn from_reader<'a, R: Read + BufRead>(mut reader: R) -> Result<Self, &'a str> {
        if !crate::skip_string(reader, "ZenGin Archive") {
            return Err("Not a valid format.");
        }
        if !crate::skip_string(reader, "ver") {
            return Err("Not a valid header.");
        }
        // Version should always be 1
        let version = crate::read_i32_ascii(reader)?;

        // Skip archiver type
        crate::skip_string(reader, "");

        // Read file-type, to create the right archiver implementation
        let file_kind_string = String::from_buf_reader(reader);

        let file_kind = match file_kind_string.as_str() {
            "ASCII" => Kind::Ascii,
            "BINARY" => Kind::Binary,
            "BIN_SAFE" => Kind::BinSafe,
            _ => return Err("Unsupported file format."),
        };
        let save_game = crate::read_bool_ascii(reader)?;
        let date = match crate::skip_string(reader, "date") {
            true => {
                let mut date = String::from_buf_reader(reader);
                date.push_str(" ");
                date.push_str(String::from_buf_reader(reader).as_str());
                date
            }
            false => String::new(),
        };

        let user = match crate::skip_string(reader, "user") {
            true => String::from_buf_reader(reader),
            false => String::new(),
        };

        match crate::skip_string(reader, "END") {
            true => {
                crate::skip_spaces(reader);
                Ok(Self {
                    version,
                    kind: file_kind,
                    save_game,
                    date,
                    user,
                    object_count: 0,
                })
            }
            false => Err("No END in header(1)"),
        }
    }
}
