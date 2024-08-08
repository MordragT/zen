use crate::binsafe::BinSafeHeader;
use crate::header::{ArchiveHeader, ArchiveKind};

use super::read::BinaryRead;
use super::{error::*, BinaryIoReader, BinarySliceReader};
use serde::de::{Deserializer, Visitor};
use serde::Deserialize;
use std::io;

/// Decode Zengin Binary Archives
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BinaryDecoder<R> {
    reader: R,
    size_stack: Vec<usize>,
}

impl<R> BinaryDecoder<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            size_stack: Vec::new(),
        }
    }

    pub fn push_size(&mut self, size: usize) {
        self.size_stack.push(size)
    }

    pub fn pop_size(&mut self) -> Option<usize> {
        self.size_stack.pop()
    }
}

impl<R> BinaryDecoder<BinaryIoReader<R>>
where
    R: io::BufRead + io::Seek,
{
    pub fn from_reader(reader: R) -> Self {
        Self {
            reader: BinaryIoReader::new(reader),
            size_stack: Vec::new(),
        }
    }
}

impl<'a> BinaryDecoder<BinarySliceReader<'a>> {
    pub fn from_slice(slice: &'a [u8]) -> Self {
        Self {
            reader: BinarySliceReader::new(slice),
            size_stack: Vec::new(),
        }
    }
}

impl<R> BinaryDecoder<R>
where
    R: BinaryRead,
{
    pub fn position(&mut self) -> io::Result<u64> {
        self.reader.position()
    }

    pub fn set_position(&mut self, pos: u64) -> io::Result<()> {
        self.reader.set_position(pos)
    }

    pub fn offset_position(&mut self, n: i64) -> io::Result<()> {
        self.reader.offset_position(n)
    }

    pub fn decode<'de, T: Deserialize<'de>>(&'de mut self) -> BinaryResult<T> {
        T::deserialize(self)
    }

    pub fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.reader.read_bytes(buf)
    }
}

impl<R> BinaryDecoder<R>
where
    R: BinaryRead,
{
    fn eat(&mut self, expected: &[u8]) -> BinaryResult<bool> {
        let mut buf = vec![0; expected.len()];
        self.read_bytes(&mut buf)?;

        if expected == buf {
            Ok(true)
        } else {
            self.offset_position(-(expected.len() as i64))?;
            Ok(false)
        }
    }

    fn eat_whitespaces(&mut self) -> BinaryResult<()> {
        loop {
            match self.reader.peek()? {
                Some(b' ') | Some(b'\r') | Some(b'\t') | Some(b'\n') | Some(0) => {
                    self.reader.offset_position(1)?
                }
                _ => break,
            }
        }

        Ok(())
    }

    fn eat_until(&mut self, byte: u8) -> BinaryResult<Vec<u8>> {
        let mut result = Vec::new();

        loop {
            let peek = self.reader.peek()?.ok_or(BinaryError::UnexpectedEoF)?;
            if peek == byte {
                return Ok(result);
            } else {
                result.push(byte)
            }
        }
    }

    pub fn decode_header(&mut self) -> BinaryResult<ArchiveHeader> {
        if !self.eat(b"ZenGin Archive\n")? {
            return Err(BinaryError::InvalidHeader);
        }

        // Version should always be 1
        if !self.eat(b"ver 1\n")? {
            return Err(BinaryError::InvalidHeader);
        }
        let version = 1;

        // Skip optional Archiver type
        if !self.eat(b"zCArchiverGeneric")? && !self.eat(b"zCArchiverBinSafe")? {
            println!("Optional archiver type not declared, maybe non default archiver type ?");
        }

        self.eat_whitespaces()?;

        // File type
        let kind = self.eat_until(b'\n')?;
        let kind = match kind.as_slice() {
            b"ASCII" => ArchiveKind::Ascii,
            b"BINARY" => ArchiveKind::Binary,
            b"BIN_SAFE" => ArchiveKind::BinSafe,
            _ => ArchiveKind::Unknown,
        };
        self.eat(b"\n")?;

        let save_game = if self.eat(b"saveGame ")? {
            match self.reader.next()?.ok_or(BinaryError::UnexpectedEoF)? {
                b'0' => false,
                b'1' => true,
                _ => return Err(BinaryError::UnexpectedByte),
            }
        } else {
            return Err(BinaryError::ExpectedSaveGame);
        };
        self.eat(b"\n")?;

        let date = if self.eat(b"date ")? {
            let date = self.eat_until(b'\n')?;
            Some(String::from_utf8_lossy(&date).to_string())
        } else {
            None
        };
        self.eat(b"\n")?;

        let user = if self.eat(b"user")? {
            let user = self.eat_until(b'\n')?;
            Some(String::from_utf8_lossy(&user).to_string())
        } else {
            None
        };
        self.eat(b"\n")?;

        self.eat(b"END\n")?;

        let object_count = if kind != ArchiveKind::BinSafe {
            let count = if self.eat(b"objects ")? {
                let bytes = self.eat_until(b'\n')?;
                i32::from_str_radix(&String::from_utf8_lossy(&bytes), 10).unwrap()
            } else {
                return Err(BinaryError::UnexpectedByte);
            };

            self.eat_whitespaces()?;
            if !self.eat(b"END\n")? {
                return Err(BinaryError::UnexpectedByte);
            }
            self.eat_whitespaces()?;
            count
        } else {
            let binsafe = self.decode::<BinSafeHeader>()?;
            binsafe.object_count as i32
        };

        let header = ArchiveHeader {
            version,
            kind,
            save_game,
            date,
            user,
            object_count,
        };

        Ok(header)
    }
}

impl<R> BinaryDecoder<R>
where
    R: BinaryRead,
{
    fn parse_bool(&mut self) -> BinaryResult<bool> {
        let byte = self.reader.next()?.ok_or(BinaryError::UnexpectedEoF)?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(BinaryError::ExpectedBool),
        }
    }

    fn parse_char(&mut self) -> BinaryResult<char> {
        let byte = self.reader.next()?.ok_or(BinaryError::UnexpectedEoF)?;
        Ok(byte as char)
    }

    fn parse_u8(&mut self) -> BinaryResult<u8> {
        let byte = self.reader.next()?.ok_or(BinaryError::UnexpectedEoF)?;
        Ok(byte)
    }

    fn parse_u16(&mut self) -> BinaryResult<u16> {
        let bytes = self
            .reader
            .next_chunk::<2>()?
            .ok_or(BinaryError::UnexpectedEoF)?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn parse_u32(&mut self) -> BinaryResult<u32> {
        let bytes = self
            .reader
            .next_chunk::<4>()?
            .ok_or(BinaryError::UnexpectedEoF)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn parse_u64(&mut self) -> BinaryResult<u64> {
        let bytes = self
            .reader
            .next_chunk::<8>()?
            .ok_or(BinaryError::UnexpectedEoF)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn parse_i8(&mut self) -> BinaryResult<i8> {
        let byte = self.reader.next()?.ok_or(BinaryError::UnexpectedEoF)?;
        Ok(i8::from_le_bytes([byte]))
    }

    fn parse_i16(&mut self) -> BinaryResult<i16> {
        let bytes = self
            .reader
            .next_chunk::<2>()?
            .ok_or(BinaryError::UnexpectedEoF)?;
        Ok(i16::from_le_bytes(bytes))
    }

    fn parse_i32(&mut self) -> BinaryResult<i32> {
        let bytes = self
            .reader
            .next_chunk::<4>()?
            .ok_or(BinaryError::UnexpectedEoF)?;
        Ok(i32::from_le_bytes(bytes))
    }

    fn parse_i64(&mut self) -> BinaryResult<i64> {
        let bytes = self
            .reader
            .next_chunk::<8>()?
            .ok_or(BinaryError::UnexpectedEoF)?;
        Ok(i64::from_le_bytes(bytes))
    }

    fn parse_f32(&mut self) -> BinaryResult<f32> {
        let bytes = self
            .reader
            .next_chunk::<4>()?
            .ok_or(BinaryError::UnexpectedEoF)?;
        Ok(f32::from_le_bytes(bytes))
    }

    fn parse_f64(&mut self) -> BinaryResult<f64> {
        let bytes = self
            .reader
            .next_chunk::<8>()?
            .ok_or(BinaryError::UnexpectedEoF)?;
        Ok(f64::from_le_bytes(bytes))
    }

    fn parse_str(&mut self) -> BinaryResult<String> {
        let mut res = String::new();
        loop {
            match self.reader.next()?.ok_or(BinaryError::UnexpectedEoF)? {
                0 | 10 => return Ok(res),
                x if x > 0 && x < 128 => res.push(x as char),
                x => return Err(BinaryError::ExpectedAsciiChar(x)),
            }
        }
    }
}

impl<'de, R> Deserializer<'de> for &mut BinaryDecoder<R>
where
    R: BinaryRead + 'de,
{
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
        visitor.visit_bool(self.parse_bool()?)
    }
    fn deserialize_char<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_char(self.parse_char()?)
    }
    fn deserialize_u8<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }
    fn deserialize_u16<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }
    fn deserialize_u32<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }
    fn deserialize_u64<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_u64()?)
    }
    fn deserialize_i8<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }
    fn deserialize_i16<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }
    fn deserialize_i64<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_i64()?)
    }
    fn deserialize_f32<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }
    fn deserialize_f64<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }
    fn deserialize_str<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.parse_str()?.as_str())
    }
    fn deserialize_string<V>(self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.parse_str()?)
    }
    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(Access::new(&mut self, len))
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
        visitor.visit_seq(Access::new(&mut self, len))
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
        visitor.visit_seq(Access::new(&mut self, fields.len()))
    }
    fn deserialize_byte_buf<V>(mut self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let size = self.pop_size().ok_or(BinaryError::MissingBufferSize)?;
        visitor.visit_seq(Access::new(&mut self, size))
    }
    fn deserialize_bytes<V>(mut self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let size = self.pop_size().ok_or(BinaryError::MissingBufferSize)?;
        visitor.visit_seq(Access::new(&mut self, size))
    }
    fn deserialize_seq<V>(mut self, visitor: V) -> BinaryResult<V::Value>
    where
        V: Visitor<'de>,
    {
        let size = self.pop_size().ok_or(BinaryError::MissingBufferSize)?;
        visitor.visit_seq(Access::new(&mut self, size))
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

struct Access<'a, R>
where
    R: BinaryRead,
{
    decoder: &'a mut BinaryDecoder<R>,
    size: usize,
}

impl<'a, R> Access<'a, R>
where
    R: BinaryRead,
{
    pub fn new(decoder: &'a mut BinaryDecoder<R>, size: usize) -> Self {
        Self { decoder, size }
    }
}

impl<'a, 'de, R: BinaryRead + 'de> serde::de::SeqAccess<'de> for Access<'a, R> {
    type Error = BinaryError;

    fn next_element_seed<T>(&mut self, seed: T) -> BinaryResult<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.size > 0 {
            self.size -= 1;
            let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.decoder)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.size)
    }
}
