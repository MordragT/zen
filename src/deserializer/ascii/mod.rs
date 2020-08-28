use super::{Header, Kind, Position};
use error::{Error, ErrorCode, Result};
use parse::Bytes;
use serde::de::{self, Deserializer, Visitor};
use std::convert::TryFrom;
use std::io::Read;

pub mod error;
pub mod parse;
pub struct AsciiDeserializer<'a> {
    header: Header,
    bytes: Bytes<'a>,
}

impl<'a> AsciiDeserializer<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self> {
        let mut bytes = Bytes::new(bytes);

        if !bytes.consume("ZenGin Archive\n") {
            return Err(bytes.error(ErrorCode::InvalidHeader));
        }
        if !bytes.consume("ver ") {
            return Err(bytes.error(ErrorCode::InvalidHeader));
        }
        // Version should always be 1
        let version = bytes.unchecked_int()?;

        // Skip optional Archiver type
        if !bytes.consume("zCArchiverGeneric") {
            println!("Optional archiver type not declared, maybe non default archiver type ?");
        }
        // Skip file type ASCII
        if !bytes.consume("ASCII\n") {
            return Err(bytes.error(ErrorCode::InvalidHeader));
        }

        let save_game = match bytes.consume("saveGame ") {
            true => bytes.unchecked_bool()?,
            false => return Err(bytes.error(ErrorCode::InvalidHeader)),
        };
        let date = match bytes.consume("date ") {
            true => Some(bytes.parse_string_until('\n')?),
            false => None,
        };
        let user = match bytes.consume("user ") {
            true => Some(bytes.parse_string_until('\n')?),
            false => None,
        };

        // Skip optional END
        bytes.consume("END\n");

        let object_count = match bytes.consume("objects ") {
            true => bytes.unchecked_int()?,
            false => return Err(bytes.error(ErrorCode::InvalidHeader)),
        };

        if !bytes.consume("END\n") {
            return Err(bytes.error(ErrorCode::ExpectedAsciiHeaderEnd));
        }
        let header = Header::new(version, Kind::Ascii, save_game, date, user, object_count);
        Ok(Self { header, bytes })
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut AsciiDeserializer<'de> {
    type Error = Error;
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("any".to_owned())))
    }
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let boolean = self.bytes.bool()?;
        match boolean {
            true => visitor.visit_bool(true),
            false => visitor.visit_bool(false),
        }
    }
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.bytes.int()?;
        match i8::try_from(int) {
            Ok(i) => visitor.visit_i8(i),
            Err(e) => Err(self.bytes.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.bytes.int()?;
        match i16::try_from(int) {
            Ok(i) => visitor.visit_i16(i),
            Err(e) => Err(self.bytes.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.bytes.int()?;
        visitor.visit_i32(int)
    }
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.bytes.int()?;
        visitor.visit_i64(int as i64)
    }
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.bytes.int()?;
        match u8::try_from(int) {
            Ok(i) => visitor.visit_u8(i),
            Err(e) => Err(self.bytes.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.bytes.int()?;
        match u16::try_from(int) {
            Ok(i) => visitor.visit_u16(i),
            Err(e) => Err(self.bytes.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.bytes.int()?;
        match u32::try_from(int) {
            Ok(i) => visitor.visit_u32(i),
            Err(e) => Err(self.bytes.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let int = self.bytes.int()?;
        match u64::try_from(int) {
            Ok(i) => visitor.visit_u64(i),
            Err(e) => Err(self.bytes.error(ErrorCode::TryFromInt(e))),
        }
    }
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let float = self.bytes.float()?;
        visitor.visit_f32(float)
    }
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let float = self.bytes.float()?;
        visitor.visit_f64(float as f64)
    }
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("char".to_owned())))
    }
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let string = self.bytes.string()?;
        visitor.visit_str(string.as_str())
    }
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let string = self.bytes.string()?;
        visitor.visit_string(string)
    }
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.bytes.bytes()?;
        visitor.visit_bytes(bytes.as_slice())
    }
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.bytes.bytes()?;
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
        self.bytes.advance_for_whitespaces()?;
        if !self.bytes.consume("[") {
            return Err(self.bytes.error(ErrorCode::ExpectedStructHeader));
        }
        // Expected header not footer
        if self.bytes.peek_or_eof()? == b']' {
            return Err(self.bytes.error(ErrorCode::ExpectedStructHeader));
        }
        // ignore object name
        self.bytes.advance_until(b' ')?;

        // // reference
        // let reference = self.bytes.parse_string_until(' ')?;
        // let create_object = match reference.as_str() {
        //     "%" => true,
        //     "§" => false,
        //     _ => return Err(self.bytes.error(ErrorCode::InvalidDescriptor)),
        // };

        // check if struct name is given
        let struct_name = match self.bytes.consume("% ") {
            true => None,
            false => Some(self.bytes.parse_string_until(' ')?),
        };

        let version = self.bytes.unchecked_int()?;
        let id = match (self.bytes.peek_or_eof()? as char).to_digit(10) {
            Some(val) => {
                self.bytes.advance_single()?;
                val
            }
            None => return Err(self.bytes.error(ErrorCode::InvalidStructHeader)),
        };

        if !self.bytes.consume("]\n") {
            return Err(self.bytes.error(ErrorCode::InvalidStructHeader));
        }

        let value = visitor.visit_seq(Access::new(&mut self))?;

        self.bytes.advance_for_whitespaces()?;
        if self.bytes.consume("[]") {
            Ok(value)
        } else {
            Err(self.bytes.error(ErrorCode::ExpectedStructEnd))
        }
    }
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = visitor.visit_seq(Access::new(&mut self))?;
        self.bytes.advance_for_whitespaces()?;
        if self.bytes.peek_tuple() == Some((b'[', b']')) {
            Ok(value)
        } else {
            Err(self.bytes.error(ErrorCode::ExpectedStructEnd))
        }
    }
    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // variants an position knüpfen ?
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("Enum".to_owned())))
    }
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("Identifier".to_owned())))
    }
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("Identifier".to_owned())))
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("Map".to_owned())))
    }
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("Option".to_owned())))
    }
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self.bytes.error(ErrorCode::DeserializeNotSupported(
            "Tuple struct".to_owned(),
        )))
    }
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("unit".to_owned())))
    }
    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(self
            .bytes
            .error(ErrorCode::DeserializeNotSupported("Unit struct".to_owned())))
    }
}

struct Access<'a, 'de: 'a> {
    de: &'a mut AsciiDeserializer<'de>,
}

impl<'a, 'de> Access<'a, 'de> {
    fn new(de: &'a mut AsciiDeserializer<'de>) -> Self {
        Access { de }
    }
    fn has_element(&mut self) -> Result<bool> {
        self.de.bytes.advance_for_whitespaces()?;
        match self.de.bytes.peek_tuple() {
            Some(val) if val.0 == b'[' && val.1 == b']' => Ok(false),
            None => Err(self.de.bytes.error(ErrorCode::EndOfFile)),
            _ => Ok(true),
        }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for Access<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.has_element()? {
            let res = seed.deserialize(&mut *self.de)?;
            Ok(Some(res))
        } else {
            Ok(None)
        }
    }
}
