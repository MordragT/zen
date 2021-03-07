use super::error::*;
use super::Code;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::io::{Seek, SeekFrom};
use zen_parser::binary::{BinaryDeserializer, BinaryRead};

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub parent: i32,
    pub kind: SymbolKind,
}

// impl Symbol {
//     pub fn load<R: BinaryRead>(
//         &self,
//         deserializer: *mut BinaryDeserializer<R>,
//         symbol_table: *mut SymbolTable,
//         index: u32,
//     ) -> Result<()> {
//         let mut deserializer = unsafe { &mut *deserializer };
//         let mut symbol_table = unsafe { &mut *symbol_table };

//         let bak = deserializer.seek(SeekFrom::Current(0))?;

//         match self.kind {
//             SymbolKind::Void => return Ok(()),
//             SymbolKind::Float(_) => panic!(),
//             SymbolKind::Int(_) => panic!(),
//             SymbolKind::String(_) => panic!(),
//             SymbolKind::Class(c) => todo!(),
//             SymbolKind::Func(f) => todo!(),
//             SymbolKind::Prototype(p) => todo!(),
//             SymbolKind::Instance(i) => {
//                 let offset = i + index as usize;
//                 deserializer.seek(SeekFrom::Start(offset as u64))?;
//                 symbol_table.insert_data(offset, i32::deserialize(deserializer)?);
//             }
//         }

//         deserializer.seek(SeekFrom::Start(bak))?;
//         Ok(())
//     }
// }

#[derive(Debug)]
pub enum SymbolKind {
    Void,
    Float(Vec<i32>),
    Int(Vec<i32>),
    String(Vec<String>),
    Class(usize),
    Func(usize),
    Prototype(usize),
    Instance(usize),
}

impl SymbolKind {
    pub fn get_offset(&self) -> Option<usize> {
        match self {
            Self::Void | Self::Float(_) | Self::Int(_) | Self::String(_) => None,
            Self::Class(c) => Some(*c),
            Self::Func(f) => Some(*f),
            Self::Prototype(p) => Some(*p),
            Self::Instance(i) => Some(*i),
        }
    }
    pub fn get_static(&self, offset: usize) -> Option<&i32> {
        match self {
            Self::Class(_)
            | Self::Func(_)
            | Self::Prototype(_)
            | Self::Instance(_)
            | Self::Void => None,
            Self::Float(vec) => vec.get(offset),
            Self::Int(vec) => vec.get(offset),
            Self::String(_) => todo!(),
        }
    }
    pub fn get_mut_static(&mut self, offset: usize) -> Option<&mut i32> {
        match self {
            Self::Class(_)
            | Self::Func(_)
            | Self::Prototype(_)
            | Self::Instance(_)
            | Self::Void => None,
            Self::Float(vec) => vec.get_mut(offset),
            Self::Int(vec) => vec.get_mut(offset),
            Self::String(_) => todo!(),
        }
    }
}

pub struct SymbolTable {
    table: HashMap<usize, Symbol>,
    // data: HashMap<usize, i32>,
    // str_data: HashMap<usize, String>,
}

impl SymbolTable {
    pub fn new(table: HashMap<usize, Symbol>) -> Self {
        Self {
            table,
            // data: HashMap::new(),
            // str_data: HashMap::new(),
        }
    }
    pub fn insert(&mut self, address: usize, symbol: Symbol) {
        self.table.insert(address, symbol);
    }
    pub fn get(&self, offset: &usize) -> Option<&Symbol> {
        self.table.get(offset)
    }
    pub fn get_mut(&mut self, offset: &usize) -> Option<&mut Symbol> {
        self.table.get_mut(offset)
    }
    pub fn contains(&self, address: &usize) -> bool {
        self.table.contains_key(address)
    }
    // pub fn insert_data(&mut self, index: usize, element: i32) -> Option<i32> {
    //     self.data.insert(index, element)
    // }
    // pub fn insert_str(&mut self, index: usize, element: String) -> Option<String> {
    //     self.str_data.insert(index, element)
    // }
    // pub fn get_mut_data(&mut self, index: &usize) -> Option<&mut i32> {
    //     self.data.get_mut(index)
    // }
    // pub fn get_data(&self, index: &usize) -> Option<&i32> {
    //     self.data.get(index)
    // }
    // pub fn append_data(&mut self, other: Vec<i32>) -> usize {
    //     let len = self.data.len();
    //     self.data
    //         .extend(other.into_iter().enumerate().map(|(i, v)| (i + len, v)));
    //     len
    // }
    // pub fn append_str(&mut self, other: Vec<String>) -> usize {
    //     let len = self.str_data.len();
    //     self.str_data
    //         .extend(other.into_iter().enumerate().map(|(i, v)| (i + len, v)));
    //     len
    // }
    // pub unsafe fn borrow_mut(&self) -> &mut HashMap<u32, Symbol> {
    //     unsafe { self.table.get().as_mut().unwrap() }
    // }
    // pub fn borrow(&self) -> &HashMap<u32, Symbol> {
    //     unsafe { self.table.get().as_ref().unwrap() }
    // }
}

#[derive(Default)]
struct Element(u32);

impl Element {
    pub fn new(val: u32) -> Element {
        Element(val)
    }
    pub fn get_count(&self) -> u32 {
        self.0 & 0xfff // 0 bis 11 einschließlich
    }
    pub fn get_kind(&self) -> Kind {
        (((self.0 & 0xf000) >> 12) as u8).try_into().unwrap() // 12 bis 15 einschließlich
    }
    pub fn has_flag(&self, flag: Flag) -> bool {
        ((self.0 & 0x3F0000) >> 16) as u8 & flag as u8 == flag as u8 // 16 bis 21 einschließlich
    }
    pub fn get_space(&self) -> u32 {
        self.0 & 0x400000 >> 22 // 22
    }
    pub fn get_reserved(&self) -> u32 {
        self.0 & 0xFF800000 >> 23 // 22 bis 31 einscließlich
    }
}

#[derive(Default)]
struct Structure(u32);

impl Structure {
    pub fn new(val: u32) -> Structure {
        Structure(val)
    }
    pub fn get_value(&self) -> u32 {
        self.0 & 0x1ffffff // 0 bis 18 einschließlich
    }
    pub fn get_reserved(&self) -> u32 {
        self.0 & 0x303FF020 // 19 bis 31 einschließlich
    }
}
#[derive(Default)]
struct CharStructure(u32);

impl CharStructure {
    pub fn new(val: u32) -> CharStructure {
        CharStructure(val)
    }
    pub fn get_value(&self) -> u32 {
        self.0 & 0xffffffff // 0 bis 23 einschließlich
    }
    pub fn get_reserved(&self) -> u32 {
        self.0 & 0x303F0000 // 24 bis 31 einschließlich
    }
}
#[derive(Default)]
pub struct Properties {
    off_cls_ret: i32,
    element: Element,
    // (Value, Reserved)
    file_index: Structure,
    line_start: Structure,
    line_count: Structure,
    char_start: CharStructure,
    char_count: CharStructure,
}
impl Properties {
    pub fn new(
        off_cls_ret: i32,
        element: u32,
        file_index: u32,
        line_start: u32,
        line_count: u32,
        char_start: u32,
        char_count: u32,
    ) -> Properties {
        Properties {
            off_cls_ret,
            element: Element::new(element),
            file_index: Structure::new(file_index),
            line_start: Structure::new(line_start),
            line_count: Structure::new(line_count),
            char_start: CharStructure::new(char_start),
            char_count: CharStructure::new(char_count),
        }
    }
    pub fn has_flag(&self, flag: Flag) -> bool {
        self.element.has_flag(flag)
    }
    pub fn get_count(&self) -> u32 {
        self.element.get_count()
    }
    pub fn get_kind(&self) -> Kind {
        self.element.get_kind()
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Flag {
    Const = 0b00001,
    Return = 0b00010,
    ClassVar = 0b00100,
    External = 0b01000,
    Merged = 0b10000,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Void = 0,
    Float = 1,
    Int = 2,
    String = 3,
    Class = 4,
    Func = 5,
    Prototype = 6,
    Instance = 7,
}

impl Default for Kind {
    fn default() -> Self {
        Self::Void
    }
}

impl TryFrom<u8> for Kind {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            x if x == Kind::Void as u8 => Ok(Kind::Void),
            x if x == Kind::Float as u8 => Ok(Kind::Float),
            x if x == Kind::Int as u8 => Ok(Kind::Int),
            x if x == Kind::String as u8 => Ok(Kind::String),
            x if x == Kind::Class as u8 => Ok(Kind::Class),
            x if x == Kind::Func as u8 => Ok(Kind::Func),
            x if x == Kind::Prototype as u8 => Ok(Kind::Prototype),
            x if x == Kind::Instance as u8 => Ok(Kind::Instance),
            _ => Err(()),
        }
    }
}
