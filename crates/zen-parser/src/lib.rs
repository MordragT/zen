use ascii::{AsciiDeserializer, AsciiRead, AsciiErrorCode};
use binary::{BinaryDeserializer, BinaryRead};
pub use error::Error;
use error::Result;
use serde::Deserialize;

pub mod ascii;
pub mod binary;
pub mod binsafe;
mod error;
pub mod prelude;

/// Possible filetypes this vdfs-file can have
#[derive(Debug, PartialEq)]
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

// TODO move to archive

/// Reads the header and returns it
pub fn read_header<R: BinaryRead + AsciiRead>(mut reader: R) -> Result<Header> {
    if !reader.consume("ZenGin Archive\n")? {
        return Err(reader.error(AsciiErrorCode::InvalidHeader).into());
    }

    if !reader.consume("ver ")? {
        return Err(reader.error(AsciiErrorCode::InvalidHeader).into());
    }
    // Version should always be 1
    let version = reader.unchecked_int()?;

    // Skip optional Archiver type
    if !reader.consume("zCArchiverGeneric")? && !reader.consume("zCArchiverBinSafe")? {
        println!("Optional archiver type not declared, maybe non default archiver type ?");
    }
    <R as AsciiRead>::consume_whitespaces(&mut reader)?;
    // File type
    let kind = reader.string_until(b'\n')?;
    let kind = match kind.as_str() {
        "ASCII" => Kind::Ascii,
        "BINARY" => Kind::Binary,
        "BIN_SAFE" => Kind::BinSafe,
        _ => Kind::Unknown,
    };
    log::debug!("Header Kind: {kind:?}");

    let save_game = match reader.consume("saveGame ")? {
        true => reader.unchecked_bool()?,
        false => {
            let e = reader.string_until_whitespace()?;
            return Err(reader
                .error(AsciiErrorCode::Expected(format!("'saveGame ', got: '{}'", e)))
                .into());
        }
    };
    let date = match reader.consume("date ")? {
        true => Some(reader.string_until(b'\n')?),
        false => None,
    };
    let user = match reader.consume("user ")? {
        true => Some(reader.string_until(b'\n')?),
        false => None,
    };
    // Skip optional END
    reader.consume("END\n")?;

    let object_count = if kind != Kind::BinSafe {
        let count = match reader.consume("objects ")? {
            true => reader.unchecked_int()?,
            false => {
                let e = reader.string_until_whitespace()?;
                return Err(reader
                    .error(AsciiErrorCode::Expected(format!("'objects ', got: '{}'", e)))
                    .into());
            }
        };
        <R as AsciiRead>::consume_whitespaces(&mut reader)?;
        if !reader.consume("END\n")? {
            return Err(reader.error(AsciiErrorCode::ExpectedAsciiHeaderEnd).into());
        }
        <R as AsciiRead>::consume_whitespaces(&mut reader)?;
        count
    } else {
        let mut deserializer = BinaryDeserializer::from(&mut reader);
        let binsafe = dbg!(<binsafe::BinSafeHeader>::deserialize(&mut deserializer)?);
        binsafe.object_count as i32
    };

    let header = Header::new(version, kind, save_game, date, user, object_count);
    Ok(header)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::read_header;

    #[test]
    fn test_read_header() {
        let header = "ZenGin Archive\nver 1\nzCArchiverGeneric\nBINARY\nsaveGame 0\nEND\nobjects 4        \nEND\n  ";
        let cursor = Cursor::new(header.as_bytes());

        let header = read_header(cursor).unwrap();
        println!("Header: {header:?}");
    }
}
