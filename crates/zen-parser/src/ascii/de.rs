use super::error::*;
use super::read::AsciiRead;
use crate::binary::{BinaryDeserializer, BinaryRead};
use serde::de::{self, Deserializer, Visitor};
use std::convert::TryFrom;
use std::io::{Read, Seek, SeekFrom};

pub struct AsciiDeserializer<R> {
    pub parser: R,
}

impl<R: AsciiRead> From<R> for AsciiDeserializer<R> {
    fn from(parser: R) -> Self {
        Self { parser }
    }
}

impl<R: BinaryRead + AsciiRead> From<BinaryDeserializer<R>> for AsciiDeserializer<R> {
    fn from(b: BinaryDeserializer<R>) -> Self {
        b.parser.into()
    }
}

impl<R: AsciiRead> Read for AsciiDeserializer<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.parser.read(buf)
    }
}

impl<R: AsciiRead> Seek for AsciiDeserializer<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.parser.seek(pos)
    }
}

impl<R: AsciiRead> AsciiDeserializer<R> {
    fn has_element(&mut self) -> Result<bool> {
        self.parser.consume_whitespaces()?;
        match self.parser.peek_tuple() {
            Ok(val) if val.0 == b'[' && val.1 == b']' => Ok(false),
            Err(_) => Err(self.parser.error(ErrorCode::EndOfFile)),
            _ => Ok(true),
        }
    }
}

impl<'de, 'a, R: AsciiRead> Deserializer<'de> for &'a mut AsciiDeserializer<R> {
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
        let bytes = self.parser.raw()?;
        visitor.visit_bytes(bytes.as_slice())
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.parser.raw()?;
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
        self.parser.consume_whitespaces()?;
        if !self.parser.consume("[")? {
            return Err(self.parser.error(ErrorCode::ExpectedStructHeader));
        }
        // Expected header not footer
        if self.parser.peek()? == b']' {
            return Err(self.parser.error(ErrorCode::ExpectedStructHeader));
        }
        // ignore object name
        self.parser.consume_until(b' ')?;

        // // reference
        // let reference = self.parser.string_until(' ')?;
        // let create_object = match reference.as_str() {
        //     "%" => true,
        //     "§" => false,
        //     _ => return Err(self.parser.error(ErrorCode::InvalidDescriptor)),
        // };

        // check if struct name is given
        let _struct_name = match self.parser.consume("% ")? {
            true => None,
            false => Some(self.parser.string_until(b' ')?),
        };

        let _version = self.parser.unchecked_int()?;
        let _id = match (self.parser.peek()? as char).to_digit(10) {
            Some(val) => {
                self.parser.seek(SeekFrom::Current(1))?;
                val
            }
            None => return Err(self.parser.error(ErrorCode::InvalidStructHeader)),
        };

        if !self.parser.consume("]\n")? {
            return Err(self.parser.error(ErrorCode::InvalidStructHeader));
        }

        let value = visitor.visit_seq(&mut self)?;

        self.parser.consume_whitespaces()?;
        if self.parser.consume("[]")? {
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
        self.parser.consume_whitespaces()?;
        if self.parser.peek_tuple()? == (b'[', b']') {
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

impl<'de, 'a, R: AsciiRead> de::SeqAccess<'de> for AsciiDeserializer<R> {
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
