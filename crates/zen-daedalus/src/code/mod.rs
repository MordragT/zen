use crate::data::Data;
use error::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{Seek, SeekFrom},
};

use symbol::{Flag, Kind, Properties, Symbol, SymbolKind, SymbolTable};

use zen_parser::prelude::*;

pub mod error;
pub mod symbol;

pub struct Code<R: BinaryRead> {
    pub deserializer: BinaryDeserializer<R>,
    pub data: Vec<Data>,
    pub symbol_table: SymbolTable,
    len: usize,
}

impl<R: BinaryRead> Code<R> {
    pub fn open(reader: R) -> Result<Self> {
        let mut deserializer = BinaryDeserializer::from(reader);

        let version = u8::deserialize(&mut deserializer)?;
        let symbol_count = u32::deserialize(&mut deserializer)?;

        let addresses = (0..symbol_count)
            .into_iter()
            .map(|_| {
                let address = u32::deserialize(&mut deserializer)?;
                Ok(address)
            })
            .collect::<Result<Vec<u32>>>()?;

        let symbols = (0..symbol_count)
            .into_iter()
            .map(|_| {
                let named = u32::deserialize(&mut deserializer)?;
                let name = if named != 0 {
                    let first = u8::deserialize(&mut deserializer)?;
                    let mut name = String::deserialize(&mut deserializer)?;
                    if first != 0xff {
                        name.insert(0, first as char);
                    }
                    name
                } else {
                    "".to_string()
                };
                let properties = Properties::new(
                    i32::deserialize(&mut deserializer)?,
                    u32::deserialize(&mut deserializer)?,
                    u32::deserialize(&mut deserializer)?,
                    u32::deserialize(&mut deserializer)?,
                    u32::deserialize(&mut deserializer)?,
                    u32::deserialize(&mut deserializer)?,
                    u32::deserialize(&mut deserializer)?,
                );
                let kind = if !properties.has_flag(Flag::ClassVar) {
                    match properties.get_kind() {
                        Kind::Float => {
                            deserializer.len_queue.push(properties.get_count() as usize);
                            SymbolKind::Float(<Vec<i32>>::deserialize(&mut deserializer)?)
                        }
                        Kind::Int => {
                            deserializer.len_queue.push(properties.get_count() as usize);
                            SymbolKind::Int(<Vec<i32>>::deserialize(&mut deserializer)?)
                        }
                        Kind::String => {
                            deserializer.len_queue.push(properties.get_count() as usize);
                            SymbolKind::String(<Vec<String>>::deserialize(&mut deserializer)?)
                        }
                        Kind::Class => SymbolKind::Class(i32::deserialize(&mut deserializer)?),
                        Kind::Func => SymbolKind::Func(i32::deserialize(&mut deserializer)?),
                        Kind::Prototype => {
                            SymbolKind::Prototype(i32::deserialize(&mut deserializer)?)
                        }
                        Kind::Instance => {
                            SymbolKind::Instance(i32::deserialize(&mut deserializer)?)
                        }
                        Kind::Void => SymbolKind::Void,
                    }
                } else {
                    // TODO change
                    SymbolKind::Void
                };

                let parent = i32::deserialize(&mut deserializer)?;

                dbg!(&name);

                Ok(Symbol { name, parent, kind })
            })
            .collect::<Result<Vec<Symbol>>>()?;

        let symbol_table = SymbolTable(
            addresses
                .into_iter()
                .zip(symbols.into_iter())
                .collect::<HashMap<u32, Symbol>>(),
        );

        // Read Stack

        let len = ((u32::deserialize(&mut deserializer)? as u64)
            - deserializer.seek(SeekFrom::Current(0))?) as usize;

        Ok(Self {
            deserializer,
            symbol_table,
            len,
            data: vec![],
        })
    }
    pub fn len(&self) -> usize {
        self.len
    }
}
