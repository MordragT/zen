use super::error::*;
use super::read::BinaryRead;
use crate::ascii::{AsciiDeserializer, AsciiRead};
use serde::de::{Deserializer, Visitor};
use std::io::{Read, Seek, SeekFrom};
/// Deserialize Zengin Binary Archives
#[derive(Debug)]
pub struct BinaryDeserializer<R: BinaryRead> {
    pub(crate) parser: R,
    pub len_queue: Vec<usize>,
}

impl<R: BinaryRead> From<R> for BinaryDeserializer<R> {
    fn from(parser: R) -> Self {
        Self {
            parser,
            len_queue: vec![],
        }
    }
}

impl<R: BinaryRead + AsciiRead> From<AsciiDeserializer<R>> for BinaryDeserializer<R> {
    fn from(b: AsciiDeserializer<R>) -> Self {
        b.parser.into()
    }
}

impl<R: BinaryRead> Read for BinaryDeserializer<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.parser.read(buf)
    }
}

impl<R: BinaryRead> Seek for BinaryDeserializer<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.parser.seek(pos)
    }
}

impl<'de, R: BinaryRead + 'de> Deserializer<'de> for &mut BinaryDeserializer<R> {
    type Error = BinaryError;
    fn deserialize_any<V>(self, _visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_bool<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parser.bool()?)
    }
    fn deserialize_char<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.parser.char()?)
    }
    fn deserialize_u8<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parser.u8()?)
    }
    fn deserialize_u16<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parser.u16()?)
    }
    fn deserialize_u32<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parser.u32()?)
    }
    fn deserialize_u64<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parser.u64()?)
    }
    fn deserialize_i8<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parser.i8()?)
    }
    fn deserialize_i16<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parser.i16()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parser.i32()?)
    }
    fn deserialize_i64<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parser.i64()?)
    }
    fn deserialize_f32<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parser.f32()?)
    }
    fn deserialize_f64<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parser.f64()?)
    }
    fn deserialize_str<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.parser.string()?.as_str())
    }
    fn deserialize_string<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.parser.string()?)
    }
    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(Access {
            deserializer: &mut self,
            len,
        })
    }
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_tuple_struct<V>(
        mut self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(Access {
            deserializer: &mut self,
            len,
        })
    }
    fn deserialize_struct<V>(
        mut self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(Access {
            deserializer: &mut self,
            len: fields.len(),
        })
    }
    fn deserialize_byte_buf<V>(mut self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = match &self.len_queue.pop() {
            Some(l) => *l,
            None => 0,
        };
        visitor.visit_seq(Access {
            deserializer: &mut self,
            len,
        })
    }
    fn deserialize_bytes<V>(mut self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = match &self.len_queue.pop() {
            Some(l) => *l,
            None => 0,
        };
        visitor.visit_seq(Access {
            deserializer: &mut self,
            len,
        })
    }
    fn deserialize_seq<V>(mut self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = match &self.len_queue.pop() {
            Some(l) => *l,
            None => 0,
        };
        visitor.visit_seq(Access {
            deserializer: &mut self,
            len,
        })
    }
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_identifier<V>(self, _visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_ignored_any<V>(self, _visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_map<V>(self, _visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_option<V>(self, _visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_unit<V>(self, _visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct Access<'a, R: BinaryRead + 'a> {
    deserializer: &'a mut BinaryDeserializer<R>,
    len: usize,
}

impl<'a, 'de, R: BinaryRead + 'de> serde::de::SeqAccess<'de> for Access<'a, R> {
    type Error = BinaryError;

    fn next_element_seed<T>(&mut self, seed: T) -> BinaryResult<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}
