use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub parent: i32,
    pub kind: SymbolKind,
}

#[derive(Debug)]
pub enum SymbolKind {
    Void,
    Float(Vec<i32>),
    Int(Vec<i32>),
    String(Vec<String>),
    Class(i32),
    Func(i32),
    Prototype(i32),
    Instance(i32),
}

pub struct SymbolTable(pub HashMap<u32, Symbol>);

impl SymbolTable {
    pub fn get(&self, idx: &u32, arr_idx: &u32) -> Option<&i32> {
        match self.0.get(idx) {
            Some(symbol) => match &symbol.kind {
                SymbolKind::Void => None,
                SymbolKind::Float(f) => f.get(*arr_idx as usize),
                SymbolKind::Int(i) => i.get(*arr_idx as usize),
                SymbolKind::String(s) => todo!(),
                SymbolKind::Class(c) => todo!(),
                SymbolKind::Func(f) => todo!(),
                SymbolKind::Prototype(p) => todo!(),
                SymbolKind::Instance(i) => todo!(),
            },
            None => None,
        }
    }
    pub fn insert(&mut self, idx: &u32, arr_idx: &u32, value: i32) {
        match self.0.get_mut(idx) {
            Some(symbol) => match &mut symbol.kind {
                SymbolKind::Void => panic!(),
                SymbolKind::Float(f) => f.insert(*arr_idx as usize, value),
                SymbolKind::Int(i) => i.insert(*arr_idx as usize, value),
                SymbolKind::String(s) => todo!(),
                SymbolKind::Class(c) => todo!(),
                SymbolKind::Func(f) => todo!(),
                SymbolKind::Prototype(p) => todo!(),
                SymbolKind::Instance(i) => todo!(),
            },
            None => panic!(),
        }
    }
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

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
