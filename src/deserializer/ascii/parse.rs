use super::{Error, ErrorCode, Result};
use super::{Header, Position, Value};
use std::convert::Into;
use std::str;

pub struct Bytes<'a> {
    bytes: &'a [u8],
    line: usize,
    column: usize,
}

impl<'a> Bytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            line: 1,
            column: 1,
        }
    }
    pub fn any_name(&mut self) -> Result<String> {
        self.parse_string_until('=')
    }
    pub fn any_value(&mut self) -> Result<Value> {
        let kind = self.parse_string_until(':')?;
        let value = self.parse_string_until('\n')?;
        match kind.as_str() {
            "string" => Ok(Value::String(value)),
            "int" => Ok(Value::Int(match i32::from_str_radix(value.as_str(), 10) {
                Ok(i) => i,
                Err(e) => return Err(self.error(e.into())),
            })),
            "float" => Ok(Value::Float(match value.parse::<f32>() {
                Ok(f) => f,
                Err(e) => return Err(self.error(e.into())),
            })),
            "bool" => Ok(Value::Bool(match value.as_str() {
                "1" => true,
                "0" => false,
                _ => return Err(self.error(ErrorCode::ParseBoolError)),
            })),
            "raw" => Ok(Value::Raw(Vec::from(value.as_bytes()))),
            _ => Err(self.error(ErrorCode::UnknownValueKind)),
        }
    }
    pub fn parse_string_until(&mut self, char: char) -> Result<String> {
        let result = String::new();
        while let c = match self.peek() {
            Some(c) => c as char,
            None => return Err(self.error(ErrorCode::Eof)),
        } {
            self.advance_single();
            match c {
                char => return Ok(result),
                _ => result.push(c),
            }
        }
        Err(self.error(ErrorCode::Eof))
    }
    pub fn parse_until(&mut self, byte: u8) -> Result<Vec<u8>> {
        let result = vec![];
        while let c = match self.peek() {
            Some(c) => c,
            None => return Err(self.error(ErrorCode::Eof)),
        } {
            self.advance_single();
            match c {
                byte => return Ok(result),
                _ => result.push(c),
            }
        }
        Err(self.error(ErrorCode::Eof))
    }
    pub fn advance(&mut self, bytes: usize) -> Result<()> {
        for _ in 0..bytes {
            self.advance_single()?;
        }

        Ok(())
    }

    pub fn advance_single(&mut self) -> Result<()> {
        if self.peek_or_eof()? == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.bytes = &self.bytes[1..];

        Ok(())
    }
    pub fn consume(&mut self, s: &str) -> bool {
        if self.test_for(s) {
            let _ = self.advance(s.len());
            true
        } else {
            false
        }
    }
    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(0).cloned()
    }
    pub fn peek_or_eof(&self) -> Result<u8> {
        self.bytes
            .get(0)
            .cloned()
            .ok_or_else(|| self.error(ErrorCode::Eof))
    }
    pub fn error(&self, kind: ErrorCode) -> Error {
        Error {
            code: kind,
            position: Position {
                line: self.line,
                column: self.column,
            },
        }
    }
    fn test_for(&self, s: &str) -> bool {
        s.bytes()
            .enumerate()
            .all(|(i, b)| self.bytes.get(i).map_or(false, |t| *t == b))
    }
}
