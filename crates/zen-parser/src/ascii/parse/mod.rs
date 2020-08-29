use super::error::*;
use super::Position;
use std::convert::Into;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::str;

mod read;

pub use read::AsciiRead;

//pub struct AsciiCursor<'a>(Cursor<&'a [u8]>);

impl<'a> AsciiRead<'a> for Cursor<&'a [u8]> {
    fn error(&self, kind: ErrorCode) -> Error {
        Error {
            code: kind,
            position: Position { line: 0, column: 0 },
        }
    }
    fn test_for(&mut self, s: &str) -> bool {
        let mut res = true;
        for c in s.chars() {
            match self.peek() {
                Some(v) => {
                    if v != c as u8 {
                        res = false;
                        break;
                    }
                }
                None => {
                    res = false;
                    break;
                }
            }
        }
        res
    }
    fn peek(&mut self) -> Option<u8> {
        let mut buf = [0_u8; 1];
        match self.read_exact(&mut buf) {
            Ok(_) => (),
            Err(_) => return None,
        };
        self.seek(SeekFrom::Current(-1)).unwrap();
        Some(u8::from_le_bytes(buf))
    }
    fn peek_tuple(&mut self) -> Option<(u8, u8)> {
        let mut buf = [0_u8; 1];
        match self.read_exact(&mut buf) {
            Ok(_) => (),
            Err(_) => return None,
        };
        let first = u8::from_le_bytes(buf);
        match self.read_exact(&mut buf) {
            Ok(_) => (),
            Err(_) => return None,
        };
        let second = u8::from_le_bytes(buf);
        self.seek(SeekFrom::Current(-2)).unwrap();
        Some((first, second))
    }
    fn peek_or_eof(&mut self) -> Result<u8> {
        let mut buf = [0_u8; 1];
        match self.read_exact(&mut buf) {
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        };
        self.seek(SeekFrom::Current(-1)).unwrap();
        Ok(u8::from_le_bytes(buf))
    }
    fn advance_single(&mut self) -> Result<()> {
        match self.seek(SeekFrom::Current(1)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct Bytes<'a> {
    bytes: &'a [u8],
    line: usize,
    column: usize,
}

impl<'a> AsciiRead<'a> for Bytes<'a> {
    fn error(&self, kind: ErrorCode) -> Error {
        Error {
            code: kind,
            position: Position {
                line: self.line,
                column: self.column,
            },
        }
    }
    fn test_for(&mut self, s: &str) -> bool {
        s.bytes()
            .enumerate()
            .all(|(i, b)| self.bytes.get(i).map_or(false, |t| *t == b))
    }
    fn peek(&mut self) -> Option<u8> {
        self.bytes.get(0).cloned()
    }
    fn peek_tuple(&mut self) -> Option<(u8, u8)> {
        self.bytes.get(0).cloned().zip(self.bytes.get(1).cloned())
    }
    fn peek_or_eof(&mut self) -> Result<u8> {
        self.bytes
            .get(0)
            .cloned()
            .ok_or_else(|| self.error(ErrorCode::EndOfFile))
    }
    fn advance_single(&mut self) -> Result<()> {
        if self.peek_or_eof()? == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.bytes = &self.bytes[1..];

        Ok(())
    }
}

impl<'a> Bytes<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            line: 1,
            column: 1,
        }
    }
    pub fn line_envelope(&self) -> bool {
        self.column == 1
    }
}
