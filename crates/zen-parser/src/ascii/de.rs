use super::error::*;
use super::parse::{AsciiRead, Bytes};
use crate::{Header, Kind};
use serde::de::{self, Deserializer, Visitor};
use std::convert::TryFrom;
use std::io::Cursor;

pub struct AsciiDeserializer<R> {
    pub header: Option<Header>,
    parser: R,
}

impl<'a> AsciiDeserializer<Bytes<'a>> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let parser = Bytes::from_bytes(bytes);
        Self {
            header: None,
            parser,
        }
    }
}

impl<'a> AsciiDeserializer<Cursor<&'a [u8]>> {
    pub fn from_cursor(cursor: Cursor<&'a [u8]>) -> Self {
        Self {
            header: None,
            parser: cursor,
        }
    }
    pub fn into_cursor(self) -> Cursor<&'a [u8]> {
        self.parser
    }
}

impl<'de, R: AsciiRead<'de>> AsciiDeserializer<R> {
    fn has_element(&mut self) -> Result<bool> {
        self.parser.advance_for_whitespaces()?;
        match self.parser.peek_tuple() {
            Some(val) if val.0 == b'[' && val.1 == b']' => Ok(false),
            None => Err(self.parser.error(ErrorCode::EndOfFile)),
            _ => Ok(true),
        }
    }
    pub fn read_header<'a>(&mut self) -> Result<()> {
        if !self.parser.consume("ZenGin Archive\n") {
            return Err(self.parser.error(ErrorCode::InvalidHeader));
        }
        if !self.parser.consume("ver ") {
            return Err(self.parser.error(ErrorCode::InvalidHeader));
        }
        // Version should always be 1
        let version = self.parser.unchecked_int()?;
        // Skip optional Archiver type
        if !self.parser.consume("zCArchiverGeneric") {
            println!("Optional archiver type not declared, maybe non default archiver type ?");
        }
        // File type
        let kind = self.parser.parse_string_until_whitespace()?;
        let kind = match kind.as_str() {
            "ASCII" => Kind::Ascii,
            "BINARY" => Kind::Binary,
            "BIN_SAFE" => Kind::BinSafe,
            _ => return Err(self.parser.error(ErrorCode::InvalidHeader)),
        };
        let save_game = match self.parser.consume("saveGame ") {
            true => self.parser.unchecked_bool()?,
            false => return Err(self.parser.error(ErrorCode::InvalidHeader)),
        };
        let date = match self.parser.consume("date ") {
            true => Some(self.parser.parse_string_until('\n')?),
            false => None,
        };
        let user = match self.parser.consume("user ") {
            true => Some(self.parser.parse_string_until('\n')?),
            false => None,
        };
        // Skip optional END
        self.parser.consume("END\n");
        let object_count = match self.parser.consume("objects ") {
            true => self.parser.unchecked_int()?,
            false => return Err(self.parser.error(ErrorCode::InvalidHeader)),
        };
        if !self.parser.consume("END\n") {
            return Err(self.parser.error(ErrorCode::ExpectedAsciiHeaderEnd));
        }
        let header = Header::new(version, kind, save_game, date, user, object_count);
        self.header = Some(header);
        Ok(())
    }
}

impl<'de, 'a, R: AsciiRead<'de>> Deserializer<'de> for &'a mut AsciiDeserializer<R> {
    type Error = Error;
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let boolean = self.parser.bool()?;
        match boolean {
            true => visitor.visit_bool(true),
            false => visitor.visit_bool(false),
        }
    }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.parser.int()?;
        match i8::try_from(int) {
            Ok(i) => visitor.visit_i8(i),
            Err(e) => Err(self.parser.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.parser.int()?;
        match i16::try_from(int) {
            Ok(i) => visitor.visit_i16(i),
            Err(e) => Err(self.parser.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.parser.int()?;
        visitor.visit_i32(int)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.parser.int()?;
        visitor.visit_i64(int as i64)
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.parser.int()?;
        match u8::try_from(int) {
            Ok(i) => visitor.visit_u8(i),
            Err(e) => Err(self.parser.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.parser.int()?;
        match u16::try_from(int) {
            Ok(i) => visitor.visit_u16(i),
            Err(e) => Err(self.parser.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.parser.int()?;
        match u32::try_from(int) {
            Ok(i) => visitor.visit_u32(i),
            Err(e) => Err(self.parser.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.parser.int()?;
        match u64::try_from(int) {
            Ok(i) => visitor.visit_u64(i),
            Err(e) => Err(self.parser.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let float = self.parser.float()?;
        visitor.visit_f32(float)
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let float = self.parser.float()?;
        visitor.visit_f64(float as f64)
    }
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let string = self.parser.string()?;
        visitor.visit_str(string.as_str())
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let string = self.parser.string()?;
        visitor.visit_string(string)
    }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.parser.bytes()?;
        visitor.visit_bytes(bytes.as_slice())
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.parser.bytes()?;
        visitor.visit_byte_buf(bytes)
    }
    fn deserialize_struct<V>(
        mut self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.parser.advance_for_whitespaces()?;
        if !self.parser.consume("[") {
            return Err(self.parser.error(ErrorCode::ExpectedStructHeader));
        }
        // Expected header not footer
        if self.parser.peek_or_eof()? == b']' {
            return Err(self.parser.error(ErrorCode::ExpectedStructHeader));
        }
        // ignore object name
        self.parser.advance_until(b' ')?;

        // // reference
        // let reference = self.parser.parse_string_until(' ')?;
        // let create_object = match reference.as_str() {
        //     "%" => true,
        //     "§" => false,
        //     _ => return Err(self.parser.error(ErrorCode::InvalidDescriptor)),
        // };

        // check if struct name is given
        let _struct_name = match self.parser.consume("% ") {
            true => None,
            false => Some(self.parser.parse_string_until(' ')?),
        };

        let _version = self.parser.unchecked_int()?;
        let _id = match (self.parser.peek_or_eof()? as char).to_digit(10) {
            Some(val) => {
                self.parser.advance_single()?;
                val
            }
            None => return Err(self.parser.error(ErrorCode::InvalidStructHeader)),
        };

        if !self.parser.consume("]\n") {
            return Err(self.parser.error(ErrorCode::InvalidStructHeader));
        }

        let value = visitor.visit_seq(&mut self)?;

        self.parser.advance_for_whitespaces()?;
        if self.parser.consume("[]") {
            Ok(value)
        } else {
            Err(self.parser.error(ErrorCode::ExpectedStructEnd))
        }
    }
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = visitor.visit_seq(&mut self)?;
        self.parser.advance_for_whitespaces()?;
        if self.parser.peek_tuple() == Some((b'[', b']')) {
            Ok(value)
        } else {
            Err(self.parser.error(ErrorCode::ExpectedStructEnd))
        }
    }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

impl<'de, 'a, R: AsciiRead<'de>> de::SeqAccess<'de> for AsciiDeserializer<R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.has_element()? {
            let res = seed.deserialize(self)?; //serde::de::DeserializeSeed::deserialize(seed, self)?;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}
