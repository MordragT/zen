use super::super::error::*;

pub trait AsciiRead<'a> {
    fn error(&self, kind: ErrorCode) -> Error;
    fn test_for(&mut self, s: &str) -> bool;
    fn peek(&mut self) -> Option<u8>;
    fn peek_tuple(&mut self) -> Option<(u8, u8)>;
    fn peek_or_eof(&mut self) -> Result<u8>;
    fn advance_single(&mut self) -> Result<()>;

    fn advance_until(&mut self, byte: u8) -> Result<()> {
        loop {
            let b = self.peek_or_eof()?;
            self.advance_single()?;
            if b == byte {
                return Ok(());
            }
        }
    }
    fn advance(&mut self, bytes: usize) -> Result<()> {
        for _ in 0..bytes {
            self.advance_single()?;
        }

        Ok(())
    }
    fn advance_for_whitespaces(&mut self) -> Result<()> {
        loop {
            let b = self.peek_or_eof()? as char;
            if b != ' ' && b != '\r' && b != '\n' && b != '\t' {
                return Ok(());
            }
            self.advance_single()?;
        }
    }
    fn consume(&mut self, s: &str) -> bool {
        if self.test_for(s) {
            let _ = self.advance(s.len());
            true
        } else {
            false
        }
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
    fn str_to_bytes(&self, string: &str) -> Result<Vec<u8>> {
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
    fn parse_string_until(&mut self, character: char) -> Result<String> {
        let mut result = String::new();
        loop {
            let c = self.peek_or_eof()? as char;
            self.advance_single()?;
            if c == character {
                return Ok(result);
            } else {
                result.push(c);
            }
        }
    }
    fn parse_string_until_whitespace(&mut self) -> Result<String> {
        let mut result = String::new();
        loop {
            let c = self.peek_or_eof()? as char;
            self.advance_single()?;
            if c == '\n' || c == ' ' || c == '\t' {
                return Ok(result);
            } else {
                result.push(c);
            }
        }
    }
    fn any_name(&mut self) -> Result<String> {
        self.parse_string_until('=')
    }
    fn parse_until(&mut self, byte: u8) -> Result<Vec<u8>> {
        let mut result = vec![];
        loop {
            let b = self.peek_or_eof()?;
            self.advance_single()?;
            if b == byte {
                return Ok(result);
            } else {
                result.push(b);
            }
        }
    }
    fn bool(&mut self) -> Result<bool> {
        let _name = self.any_name()?;
        let kind = self.parse_string_until(':')?;
        let value = self.parse_string_until_whitespace()?;
        if kind == "bool" {
            self.str_to_bool(value.as_str())
        } else {
            Err(self.error(ErrorCode::ExpectedBool))
        }
    }
    fn unchecked_bool(&mut self) -> Result<bool> {
        let value = self.parse_string_until_whitespace()?;
        self.str_to_bool(value.as_str())
    }
    fn int(&mut self) -> Result<i32> {
        let _name = self.any_name()?;
        let kind = self.parse_string_until(':')?;
        let value = self.parse_string_until_whitespace()?;
        // Enum will be handled as integer
        if kind == "int" || kind.contains("enum") {
            self.str_to_i32(value.as_str())
        } else {
            Err(self.error(ErrorCode::ExpectedInt))
        }
    }
    fn unchecked_int(&mut self) -> Result<i32> {
        let value = self.parse_string_until_whitespace()?;
        self.str_to_i32(value.as_str())
    }
    fn float(&mut self) -> Result<f32> {
        let _name = self.any_name()?;
        let kind = self.parse_string_until(':')?;
        let value = self.parse_string_until_whitespace()?;
        if kind == "float" {
            self.str_to_f32(value.as_str())
        } else {
            Err(self.error(ErrorCode::ExpectedFloat))
        }
        // let value = self.parse_string_until_whitespace()?;
        // self.str_to_f32(value.as_str())
    }
    fn string(&mut self) -> Result<String> {
        let _name = self.any_name()?;
        let kind = self.parse_string_until(':')?;
        let value = self.parse_string_until_whitespace()?;
        if kind == "string" {
            Ok(value)
        } else {
            Err(self.error(ErrorCode::ExpectedString))
        }
    }
    fn bytes(&mut self) -> Result<Vec<u8>> {
        let _name = self.any_name()?;
        let kind = self.parse_string_until(':')?;
        let value = self.parse_string_until_whitespace()?;
        if kind == "raw" {
            self.str_to_bytes(value.as_str())
        } else {
            Err(self.error(ErrorCode::ExpectedBytes))
        }
        // let value = self.parse_string_until_whitespace()?;
        // self.str_to_bytes(value.as_str())
    }
}
