use super::{Header, Position, Value};
use error::{Error, ErrorCode, Result};
use parse::Bytes;
use serde::de::{Deserializer, Visitor};

pub mod error;
pub mod parse;
pub struct AsciiDeserializer<'a> {
    header: Header,
    bytes: Bytes<'a>,
}

impl<'de, 'a> Deserializer<'de> for &'a mut AsciiDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
    }
}
