use super::error::*;
use super::Position;
use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    mem,
};

// impl AsciiRead for Cursor<&[u8]> {}

// impl AsciiRead for Cursor<Vec<u8>> {}

// impl AsciiRead for File {}

// impl<R: AsciiRead> AsciiRead for &mut R {}

impl<R: Read + Seek> AsciiRead for R {}

/// Provides methods to read an Ascii archive
pub trait AsciiRead: Read + Seek {
    /// Return an Error with the given code
    fn error(&mut self, kind: ErrorCode) -> Error {
        let bak_seek = self.seek(SeekFrom::Current(0)).unwrap();
        self.seek(SeekFrom::Start(0)).unwrap();
        let mut line = 1;
        let mut column = 0;
        while self.seek(SeekFrom::Current(0)).unwrap() <= bak_seek {
            column = self.consume_until(b'\n').unwrap() as usize;
            line += 1;
        }
        line -= 1;
        column -= (self.seek(SeekFrom::Current(0)).unwrap() - bak_seek) as usize;
        Error {
            code: kind,
            position: Position { line, column },
        }
    }
    /// Consume a byte
    fn byte(&mut self) -> Result<u8> {
        let mut buf = [0_u8; mem::size_of::<u8>()];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }
    /// Peek for the next byte, the own position does not change
    fn peek(&mut self) -> Result<u8> {
        let b = self.byte()?;
        self.seek(SeekFrom::Current(-(mem::size_of::<u8>() as i64)))?;
        Ok(b)
    }
    /// Peek for the next two bytes, the own position does not change
    fn peek_tuple(&mut self) -> Result<(u8, u8)> {
        let mut buf = [0_u8; mem::size_of::<u8>()];
        self.read_exact(&mut buf)?;
        let first = u8::from_le_bytes(buf);
        self.read_exact(&mut buf)?;
        let second = u8::from_le_bytes(buf);
        self.seek(SeekFrom::Current(-(2 * mem::size_of::<u8>() as i64)))?;
        Ok((first, second))
    }
    /// Test if the given string matches the following string in the ascii archive
    fn test_for(&mut self, s: &str) -> Result<bool> {
        let seek_bak = self.seek(SeekFrom::Current(0))?;
        for c in s.bytes() {
            let b = self.peek()?;
            if b != c {
                self.seek(SeekFrom::Start(seek_bak))?;
                return Ok(false);
            }
            self.seek(SeekFrom::Current(1))?;
        }
        self.seek(SeekFrom::Start(seek_bak))?;
        Ok(true)
    }
    /// Consume the given string, returns false if the following string does not match
    fn consume(&mut self, s: &str) -> Result<bool> {
        if self.test_for(s)? {
            let _ = self.seek(SeekFrom::Current(s.len() as i64));
            Ok(true)
        } else {
            Ok(false)
        }
    }
    /// Consume until the given byte occurs, returns number of bytes consumed
    fn consume_until(&mut self, byte: u8) -> Result<u32> {
        let mut res = 0;
        loop {
            res += 1;
            let b = self.byte()?;
            if b == byte {
                return Ok(res);
            }
        }
    }
    /// Consumes all whitespaces until the next non whitespace char
    fn consume_whitespaces(&mut self) -> Result<()> {
        loop {
            let p = self.peek()?;
            if p == b' ' || p == b'\r' || p == b'\t' || p == b'\n' || p == 0 {
                self.seek(SeekFrom::Current(1))?;
            } else {
                break;
            }
        }
        Ok(())
    }
    /// Returns the string until the given byte
    fn string_until(&mut self, byte: u8) -> Result<String> {
        let mut res = String::new();
        loop {
            let b = self.byte()?;
            if b == byte {
                return Ok(res);
            }
            res.push(b as char);
        }
    }
    /// Returns the string until a whitespace occurs
    fn string_until_whitespace(&mut self) -> Result<String> {
        let mut res = String::new();
        loop {
            let b = self.byte()?;
            if b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' {
                return Ok(res);
            }
            res.push(b as char);
        }
    }
    /// Consumes a bool string with name and kind specified and returns its value
    fn bool(&mut self) -> Result<bool> {
        let _name = self.consume_until(b'=')?;
        let kind = self.string_until(b':')?;
        let value = self.string_until_whitespace()?;
        if kind == "bool" {
            self.str_to_bool(value.as_str())
        } else {
            Err(self.error(ErrorCode::ExpectedBool))
        }
    }
    /// Consumes only a bool string and returns its value
    fn unchecked_bool(&mut self) -> Result<bool> {
        let value = self.string_until_whitespace()?;
        self.str_to_bool(value.as_str())
    }
    /// Consumes an enum or a integer with name and kind specified and returns its value
    fn int(&mut self) -> Result<i32> {
        let _name = self.consume_until(b'=')?;
        let kind = self.string_until(b':')?;
        let value = self.string_until_whitespace()?;
        // Enum will be handled as integer
        if kind == "int" || kind.contains("enum") {
            self.str_to_i32(value.as_str())
        } else {
            Err(self.error(ErrorCode::ExpectedInt))
        }
    }
    /// Consumes only a enum or int string and returns its value
    fn unchecked_int(&mut self) -> Result<i32> {
        let value = self.string_until_whitespace()?;
        self.str_to_i32(value.as_str())
    }
    /// Consumes a float string with name and kind specified and returns its value
    fn float(&mut self) -> Result<f32> {
        let _name = self.consume_until(b'=')?;
        let kind = self.string_until(b':')?;
        let value = self.string_until_whitespace()?;
        if kind == "float" {
            self.str_to_f32(value.as_str())
        } else {
            Err(self.error(ErrorCode::ExpectedFloat))
        }
        // let value = self.string_until_whitespace()?;
        // self.str_to_f32(value.as_str())
    }
    /// Consumes a variable of the type string with name and kind specified and returns its value
    fn string(&mut self) -> Result<String> {
        let _name = self.consume_until(b'=')?;
        let kind = self.string_until(b':')?;
        let value = self.string_until_whitespace()?;
        if kind == "string" {
            Ok(value)
        } else {
            Err(self.error(ErrorCode::ExpectedString))
        }
    }
    /// Consumes a variable of the type raw vector and returns its value
    fn raw(&mut self) -> Result<Vec<u8>> {
        let _name = self.consume_until(b'=')?;
        let kind = self.string_until(b':')?;
        let value = self.string_until_whitespace()?;
        if kind == "raw" {
            self.str_to_bytes(value.as_str())
        } else {
            Err(self.error(ErrorCode::ExpectedBytes))
        }
        // let value = self.string_until_whitespace()?;
        // self.str_to_bytes(value.as_str())
    }
    /// Convert string to color rgba
    fn str_to_color(&mut self, string: &str) -> Result<(u8, u8, u8, u8)> {
        let mut nums = vec![];
        for s in string.split_ascii_whitespace() {
            match u8::from_str_radix(s, 10) {
                Ok(n) => nums.push(n),
                Err(e) => return Err(self.error(e.into())),
            }
        }
        match nums.len() > 4 {
            true => Err(self.error(ErrorCode::ParseColorError)),
            false => Ok((nums[0], nums[1], nums[2], nums[3])),
        }
    }
    /// Convert string to i32
    fn str_to_i32(&mut self, string: &str) -> Result<i32> {
        match i32::from_str_radix(string, 10) {
            Ok(i) => Ok(i),
            Err(e) => return Err(self.error(e.into())),
        }
    }
    /// Convert string to f32
    fn str_to_f32(&mut self, string: &str) -> Result<f32> {
        match string.parse::<f32>() {
            Ok(f) => Ok(f),
            Err(e) => return Err(self.error(e.into())),
        }
    }
    /// Convert string to bool
    fn str_to_bool(&mut self, string: &str) -> Result<bool> {
        match string {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => return Err(self.error(ErrorCode::ParseBoolError)),
        }
    }
    /// Convert string to bytes
    fn str_to_bytes(&mut self, string: &str) -> Result<Vec<u8>> {
        let mut buf = vec![];
        for c in string.chars() {
            match c.to_digit(16) {
                Some(val) => {
                    for b in val.to_le_bytes().iter() {
                        buf.push(*b);
                    }
                }
                None => return Err(self.error(ErrorCode::ParseBytesError)),
            }
        }
        Ok(buf)
    }
}
