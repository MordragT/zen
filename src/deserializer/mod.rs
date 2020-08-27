use serde::Deserialize;
use std::fmt;
use std::io::Read;

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
    Zero,                    // 0x0
    String(String),          // 0x1
    Int(i32),                // 0x2
    Float(f32),              // 0x3
    Byte(u8),                // 0x4
    Word(u16),               // 0x5
    Bool(bool),              // 0x6
    Vec3((f32, f32, f32)),   // 0x7
    Color((u8, u8, u8, u8)), // 0x8
    Raw(Vec<u8>),            // 0x9
    Undef1,                  // 0xA
    Undef2,                  // 0xB
    Undef3,                  // 0xC
    Undef4,                  // 0xD
    Undef5,                  // 0xE
    Undef6,                  // 0xF
    RawFloat,                // 0x10
    Enum(i32),               // 0x11
    Hash,                    // 0x12
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
    fn from_reader<'a, R: Read>(mut reader: R) -> Result<Self, &'a str> {
        if !skip_string(&mut reader, "ZenGin Archive") {
            return Err("Not a valid format.");
        }
        if !skip_string(&mut reader, "ver") {
            return Err("Not a valid header.");
        }
        // Version should always be 1
        let version = read_i32_ascii(&mut reader)?;

        // Skip archiver type
        skip_string(&mut reader, "");

        // Read file-type, to create the right archiver implementation
        let file_kind_string = bincode::deserialize_from::<&mut R, String>(&mut reader).unwrap();

        let file_kind = match file_kind_string.as_str() {
            "ASCII" => Kind::Ascii,
            "BINARY" => Kind::Binary,
            "BIN_SAFE" => Kind::BinSafe,
            _ => return Err("Unsupported file format."),
        };
        let save_game = read_bool_ascii(&mut reader)?;
        let date = match skip_string(&mut reader, "date") {
            true => {
                let mut date = bincode::deserialize_from::<&mut R, String>(&mut reader).unwrap();

                date.push_str(" ");
                date.push_str(
                    bincode::deserialize_from::<&mut R, String>(&mut reader)
                        .unwrap()
                        .as_str(),
                );
                date
            }
            false => String::new(),
        };

        let user = match skip_string(&mut reader, "user") {
            true => bincode::deserialize_from::<&mut R, String>(&mut reader).unwrap(),
            false => String::new(),
        };

        match skip_string(&mut reader, "END") {
            true => {
                skip_spaces(&mut reader);
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
            let c = bincode::deserialize_from::<&mut R, char>(&mut reader).unwrap();

            if c == '\n' || c == ' ' {
                return true;
            }
        },
        false => {
            for pattern_char in pattern.chars() {
                let c = bincode::deserialize_from::<&mut R, char>(&mut reader).unwrap();
                if c != pattern_char {
                    return false;
                }
            }
            true
        }
    }
}

pub fn read_i32_ascii<'a, R: Read>(mut reader: R) -> Result<i32, &'a str> {
    skip_spaces(&mut reader);
    let mut num = String::new();
    loop {
        let digit = bincode::deserialize_from::<&mut R, u8>(&mut reader).unwrap();
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
    skip_spaces(&mut reader);
    let val = bincode::deserialize_from::<&mut R, char>(&mut reader).unwrap();
    match val {
        '0' => Ok(false),
        '1' => Ok(true),
        _ => Err("Value is not a bool."),
    }
}

#[derive(Deserialize, Debug)]
pub struct Date {
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

#[derive(Deserialize, Debug)]
pub struct Chunk {
    pub id: u16,
    pub length: u32,
}
