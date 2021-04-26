use ascii::{error::ErrorCode, AsciiDeserializer, AsciiRead};
use binary::{BinaryDeserializer, BinaryRead};
pub use error::Error;
use error::Result;
use serde::Deserialize;

pub mod ascii;
pub mod binary;
pub mod binsafe;
pub mod error;
pub mod prelude;

/// Possible filetypes this zen-file can have
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

pub struct Reader<R: BinaryRead + AsciiRead> {
    reader: R,
}

impl<R: BinaryRead + AsciiRead> From<AsciiDeserializer<R>> for Reader<R> {
    fn from(ascii: AsciiDeserializer<R>) -> Self {
        Self {
            reader: ascii.parser,
        }
    }
}

impl<R: BinaryRead + AsciiRead> From<BinaryDeserializer<R>> for Reader<R> {
    fn from(binary: BinaryDeserializer<R>) -> Self {
        Self {
            reader: binary.parser,
        }
    }
}

impl<R: BinaryRead + AsciiRead> From<R> for Reader<R> {
    fn from(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: BinaryRead + AsciiRead> Reader<R> {
    /// Reads the header and returns it
    pub fn read_header<'a>(&mut self) -> Result<Header> {
        let mut ascii = AsciiDeserializer::from(&mut self.reader);

        if !ascii.parser.consume("ZenGin Archive\n")? {
            return Err(ascii.parser.error(ErrorCode::InvalidHeader).into());
        }
        if !ascii.parser.consume("ver ")? {
            return Err(ascii.parser.error(ErrorCode::InvalidHeader).into());
        }
        // Version should always be 1
        let version = ascii.parser.unchecked_int()?;
        // Skip optional Archiver type
        if !ascii.parser.consume("zCArchiverGeneric")?
            && !ascii.parser.consume("zCArchiverBinSafe")?
        {
            println!("Optional archiver type not declared, maybe non default archiver type ?");
        }
        <R as AsciiRead>::consume_whitespaces(&mut ascii.parser)?;
        // File type
        let kind = ascii.parser.string_until(b'\n')?;
        let kind = match kind.as_str() {
            "ASCII" => Kind::Ascii,
            "BINARY" => Kind::Binary,
            "BIN_SAFE" => Kind::BinSafe,
            _ => Kind::Unknown,
        };
        let save_game = match ascii.parser.consume("saveGame ")? {
            true => ascii.parser.unchecked_bool()?,
            false => {
                let e = ascii.parser.string_until_whitespace()?;
                return Err(ascii
                    .parser
                    .error(ErrorCode::Expected(format!("'saveGame ', got: '{}'", e)))
                    .into());
            }
        };
        let date = match ascii.parser.consume("date ")? {
            true => Some(ascii.parser.string_until(b'\n')?),
            false => None,
        };
        let user = match ascii.parser.consume("user ")? {
            true => Some(ascii.parser.string_until(b'\n')?),
            false => None,
        };
        // Skip optional END
        ascii.parser.consume("END\n")?;

        let object_count = if kind != Kind::BinSafe {
            let count = match ascii.parser.consume("objects ")? {
                true => ascii.parser.unchecked_int()?,
                false => {
                    let e = ascii.parser.string_until_whitespace()?;
                    return Err(ascii
                        .parser
                        .error(ErrorCode::Expected(format!("'objects ', got: '{}'", e)))
                        .into());
                }
            };
            <R as AsciiRead>::consume_whitespaces(&mut ascii.parser)?;
            if !ascii.parser.consume("END\n")? {
                return Err(ascii.parser.error(ErrorCode::ExpectedAsciiHeaderEnd).into());
            }
            <R as AsciiRead>::consume_whitespaces(&mut ascii.parser)?;
            count
        } else {
            let mut binary = BinaryDeserializer::from(ascii);
            let binsafe = dbg!(<binsafe::BinSafeHeader>::deserialize(&mut binary)?);
            binsafe.object_count as i32
        };

        let header = Header::new(version, kind, save_game, date, user, object_count);
        Ok(header)
    }
}
