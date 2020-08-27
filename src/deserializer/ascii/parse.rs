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
            "int" => {
                let int = self.str_to_i32(value.as_str())?;
                Ok(Value::Int(int))
            }
            "float" => {
                let float = self.str_to_f32(value.as_str())?;
                Ok(Value::Float(float))
            }
            "bool" => {
                let boolean = self.str_to_bool(value.as_str())?;
                Ok(Value::Bool(boolean))
            }
            "color" => {
                let color = self.str_to_color(value.as_str())?;
                Ok(Value::Color(color))
            }
            "raw" => Ok(Value::Raw(Vec::from(value.as_bytes()))),
            "enum" => {
                let enum_val = self.str_to_i32(value.as_str())?;
                Ok(Value::Enum(enum_val))
            }
            _ => Err(self.error(ErrorCode::UnknownValueKind(kind))),
        }
    }
    pub fn parse_string_until(&mut self, character: char) -> Result<String> {
        let mut result = String::new();
        loop {
            let c = match self.peek() {
                Some(c) => c as char,
                None => return Err(self.error(ErrorCode::Eof)),
            };
            self.advance_single();
            if c == character {
                return Ok(result);
            } else {
                result.push(c);
            }
        }
    }
    pub fn parse_until(&mut self, byte: u8) -> Result<Vec<u8>> {
        let mut result = vec![];
        loop {
            let b = match self.peek() {
                Some(b) => b,
                None => return Err(self.error(ErrorCode::Eof)),
            };
            self.advance_single();
            if b == byte {
                return Ok(result);
            } else {
                result.push(b);
            }
        }
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
    fn str_to_color(&self, string: &str) -> Result<(u8, u8, u8, u8)> {
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
    fn str_to_i32(&self, string: &str) -> Result<i32> {
        match i32::from_str_radix(string, 10) {
            Ok(i) => Ok(i),
            Err(e) => return Err(self.error(e.into())),
        }
    }
    fn str_to_f32(&self, string: &str) -> Result<f32> {
        match string.parse::<f32>() {
            Ok(f) => Ok(f),
            Err(e) => return Err(self.error(e.into())),
        }
    }
    fn str_to_bool(&self, string: &str) -> Result<bool> {
        match string {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => return Err(self.error(ErrorCode::ParseBoolError)),
        }
    }
}
