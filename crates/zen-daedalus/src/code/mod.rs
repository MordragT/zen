pub use error::Error;
use error::Result;
pub use memory::Memory;
use std::{collections::HashMap, io::SeekFrom};
use symbol::{Flag, Kind, Properties};
pub use symbol::{Symbol, SymbolKind, SymbolTable};
use zen_parser::prelude::*;

mod error;
mod memory;
mod symbol;

/// Contains the [Memory](memory::Memory) where the bytecode is loaded in.
/// It also keeps track of the current memory position
pub struct Code {
    //pub deserializer: BinaryDeserializer<R>,
    memory: Memory,
    pub symbol_table: SymbolTable,
    len: usize,
    offset: usize,
    current_instance: usize,
    memory_position: usize,
}

impl Code {
    /// Creates a new Code object from usually an opened file
    pub fn from_decoder<R>(mut decoder: BinaryDecoder<R>) -> Result<Self>
    where
        R: BinaryRead,
    {
        let _version = decoder.decode::<u8>()?;
        let symbol_count = decoder.decode::<u32>()?;

        let addresses = (0..symbol_count)
            .into_iter()
            .map(|_| {
                let address = decoder.decode::<u32>()?;
                Ok(address)
            })
            .collect::<Result<Vec<u32>>>()?;

        let symbol_table = (0..symbol_count)
            .into_iter()
            .zip(addresses.into_iter())
            .fold(
                Ok(SymbolTable::new(HashMap::new())),
                |table_res: Result<SymbolTable>, (_, address)| {
                    let mut table = match table_res {
                        Ok(h) => h,
                        Err(e) => return Err(e),
                    };

                    let named = decoder.decode::<u32>()?;
                    let name = if named != 0 {
                        let first = decoder.decode::<u8>()?;
                        let mut name = decoder.decode::<String>()?;
                        if first != 0xff {
                            name.insert(0, first as char);
                        }
                        name
                    } else {
                        "".to_owned()
                    };
                    let properties = Properties::new(
                        decoder.decode::<i32>()?,
                        decoder.decode::<u32>()?,
                        decoder.decode::<u32>()?,
                        decoder.decode::<u32>()?,
                        decoder.decode::<u32>()?,
                        decoder.decode::<u32>()?,
                        decoder.decode::<u32>()?,
                    );
                    let kind = if !properties.has_flag(Flag::ClassVar) {
                        match properties.get_kind() {
                            Kind::Float => {
                                decoder.push_size(properties.get_count() as usize);
                                SymbolKind::Float(decoder.decode::<Vec<i32>>()?)
                            }
                            Kind::Int => {
                                decoder.push_size(properties.get_count() as usize);
                                SymbolKind::Int(decoder.decode::<Vec<i32>>()?)
                            }
                            Kind::String => {
                                decoder.push_size(properties.get_count() as usize);
                                SymbolKind::String(decoder.decode::<Vec<String>>()?)
                            }
                            Kind::Class => SymbolKind::Class(decoder.decode::<u32>()? as usize),
                            Kind::Func => SymbolKind::Func(decoder.decode::<u32>()? as usize),
                            Kind::Prototype => {
                                SymbolKind::Prototype(decoder.decode::<u32>()? as usize)
                            }
                            Kind::Instance => {
                                SymbolKind::Instance(decoder.decode::<u32>()? as usize)
                            }
                            Kind::Void => SymbolKind::Void,
                        }
                    } else {
                        // TODO change
                        SymbolKind::Void
                    };

                    let parent = decoder.decode::<i32>()?;

                    table.insert(address as usize, Symbol { name, parent, kind });
                    Ok(table)
                },
            )?;

        let offset = decoder.position()? as usize;
        let len = decoder.decode::<u32>()? as usize;

        let mut memory_vec = vec![];
        let _ = decoder.read_to_end(&mut memory_vec)?;

        Ok(Self {
            memory: Memory::new(memory_vec),
            symbol_table,
            len,
            offset,
            current_instance: 0,
            memory_position: 0,
        })
    }
    /// Returns the number of instructions
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn set_current_instance(&mut self, instance: usize) {
        if self.symbol_table.contains(&instance) {
            self.current_instance = instance;
        } else {
            panic!()
        }
    }
    /// Gets a immutable reference to data at the given offset
    pub fn get(&self, offset: usize) -> Option<&i32> {
        let symbol = match self.symbol_table.get(&self.current_instance) {
            Some(s) => s,
            None => return None,
        };
        if let Some(val) = symbol.kind.get_static(offset) {
            return Some(val);
        }
        let offset = symbol.kind.get_offset().unwrap();
        self.memory.get(offset - self.offset)
    }
    /// Gets a mutable reference to data at the given offset
    pub fn get_mut(&mut self, offset: usize) -> Option<&mut i32> {
        let symbol = match self.symbol_table.get_mut(&self.current_instance) {
            Some(s) => s,
            None => return None,
        };
        if let Some(offset) = symbol.kind.get_offset() {
            self.memory.get_mut(offset - self.offset)
        } else {
            symbol.kind.get_mut_static(offset)
        }
    }
    /// Gets the next immutable reference to data in memory
    pub fn next<T>(&mut self) -> Option<&T> {
        let res = self.memory.get(self.memory_position);
        self.memory_position += std::mem::size_of::<T>();
        res
    }
    /// Gets the next mutable reference to data in memory
    pub fn next_mut<T>(&mut self) -> Option<&mut T> {
        let res = self.memory.get_mut(self.memory_position);
        self.memory_position += std::mem::size_of::<T>();
        res
    }
    // fn deserialize_at<'de, T: Deserialize<'de>>(&'de mut self, address: u64) -> Result<T> {
    //     let bak = self.deserializer.seek(SeekFrom::Current(0))?;
    //     self.deserializer.seek(SeekFrom::Start(address))?;
    //     let val = T::deserialize(&mut self.deserializer)?;
    //     self.deserializer.seek(SeekFrom::Start(bak))?;
    //     Ok(val)
    // }
    // pub fn get(&self, idx: &u32) -> Option<&i32> {
    //     let symbol = match self.symbol_table.get_symbol(&self.current_instance) {
    //         Some(s) => s,
    //         None => return None,
    //     };
    //     let offset = symbol.kind.get_offset();
    //     match self.symbol_table.get_data(&(offset + *idx as usize)) {
    //         Some(data) => return Some(data),
    //         None => (),
    //     }
    //     symbol.load(
    //         &mut self.deserializer as *mut _,
    //         &mut self.symbol_table as *mut _,
    //         *idx,
    //     );
    //     self.symbol_table.get_data(&(offset + *idx as usize))
    // }
    // pub fn get_mut(&mut self, idx: &u32) -> Option<&mut i32> {
    //     let symbol = match self.symbol_table.get_symbol(&self.current_instance) {
    //         Some(s) => s,
    //         None => return None,
    //     };
    //     let offset = symbol.kind.get_offset();
    //     match self.symbol_table.get_mut_data(&(offset + *idx as usize)) {
    //         Some(data) => return Some(data),
    //         None => (),
    //     }
    //     symbol.load(
    //         &mut self.deserializer as *mut _,
    //         &mut self.symbol_table as *mut _,
    //         *idx,
    //     );
    //     self.symbol_table.get_mut_data(&(offset + *idx as usize))
    // }
    // pub fn insert(&mut self, idx: &u32, value: i32) -> Option<i32> {
    //     match self.symbol_table.get_symbol(&self.current_instance) {
    //         Some(symbol) => match &symbol.kind {
    //             SymbolKind::Void => panic!(),
    //             SymbolKind::Float(f) => self.symbol_table.insert_data(f + *idx as usize, value),
    //             SymbolKind::Int(i) => self.symbol_table.insert_data(i + *idx as usize, value),
    //             SymbolKind::String(s) => todo!(),
    //             SymbolKind::Class(c) => todo!(),
    //             SymbolKind::Func(f) => todo!(),
    //             SymbolKind::Prototype(p) => todo!(),
    //             SymbolKind::Instance(i) => todo!(),
    //         },
    //         None => panic!(),
    //     }
    // }
}
