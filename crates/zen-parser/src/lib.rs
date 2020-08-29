use std::io::Read;

pub mod ascii;
//pub mod binsafe;
pub mod prelude;

/// Possible filetypes this zen-file can have
#[derive(Debug)]
pub enum Kind {
    Unknown,
    Ascii,
    Binary,
    BinSafe,
}
/// File Header for zen-files
#[derive(Debug)]
pub struct Header {
    version: i32,
    kind: Kind,
    save_game: bool,
    date: Option<String>,
    user: Option<String>,
    object_count: i32,
}

impl Header {
    pub fn new(
        version: i32,
        kind: Kind,
        save_game: bool,
        date: Option<String>,
        user: Option<String>,
        object_count: i32,
    ) -> Self {
        Self {
            version,
            kind,
            save_game,
            date,
            user,
            object_count,
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

// pub fn read_i32_ascii<'a, R: Read>(mut reader: R) -> Result<i32, &'a str> {
//     skip_spaces(&mut reader);
//     let mut num = String::new();
//     loop {
//         let digit = bincode::deserialize_from::<&mut R, u8>(&mut reader).unwrap();
//         if digit <= '0' as u8 || digit >= '9' as u8 {
//             break;
//         }
//         num.push(digit as char);
//     }
//     match num.is_empty() {
//         true => Err("No number found."),
//         false => Ok(i32::from_str_radix(num.as_str(), 10).unwrap()),
//     }
// }

// pub fn read_bool_ascii<'a, R: Read>(mut reader: R) -> Result<bool, &'a str> {
//     skip_spaces(&mut reader);
//     let val = bincode::deserialize_from::<&mut R, char>(&mut reader).unwrap();
//     match val {
//         '0' => Ok(false),
//         '1' => Ok(true),
//         _ => Err("Value is not a bool."),
//     }
// }
