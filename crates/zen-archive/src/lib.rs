use error::*;
use serde::Deserialize;
use std::{
    cell::UnsafeCell,
    collections::HashMap,
    io::{prelude::*, SeekFrom},
};
use zen_parser::prelude::*;

pub mod error;

const SIGNATURE_G1: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\r\n\r\n";
const SIGNATURE_G2: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\n\r\n\r";

const COMMENT_LENGTH: u64 = 256;
const SIGNATURE_LENGTH: usize = 16;
const ENTRY_NAME_LENGTH: usize = 64;
const ENTRY_DIR: u32 = 0x80000000;

#[derive(Deserialize, Debug)]
struct Properties {
    offset: u32,
    size: u32,
    kind: u32,
    attr: u32,
}
#[derive(Debug)]
pub struct Entry<'a> {
    name: String,
    properties: &'a Properties,
    pub data: Vec<u8>,
}

impl<'a> Entry<'a> {
    fn new(name: &str, properties: &'a Properties, data: Vec<u8>) -> Self {
        Self {
            name: name.to_owned(),
            properties,
            data,
        }
    }
}
#[derive(Deserialize, Debug)]
struct Header {
    signature: [u8; SIGNATURE_LENGTH],
    count: u32,
    num_files: u32,
    timestamp: u32,
    data_size: u32,
    offset: u32,
    version: u32,
}

pub struct Vdfs<R: BinaryRead> {
    deserializer: UnsafeCell<BinaryDeserializer<R>>,
    header: Header,
    entries: HashMap<String, Properties>,
}

impl<R: BinaryRead> Vdfs<R> {
    /// Creates a new Vdfs struct that holds the data of all entries
    pub fn new<'a>(reader: R) -> Result<Vdfs<R>> {
        let mut deserializer = BinaryDeserializer::from(reader);
        deserializer.seek(SeekFrom::Start(COMMENT_LENGTH))?;
        let header = Header::deserialize(&mut deserializer)?;
        if header.signature != SIGNATURE_G1 && header.signature != SIGNATURE_G2 {
            return Err(Error::UnknownSignature);
        }

        if header.version != 0x50 {
            return Err(Error::UnsupportedVersion);
        }

        deserializer.seek(SeekFrom::Start(header.offset as u64))?;

        let mut entries = HashMap::new();

        for _ in 0..header.count {
            let mut name_buf = [0_u8; ENTRY_NAME_LENGTH];
            deserializer.read_exact(&mut name_buf)?;
            let mut name = String::new();
            for c in name_buf.iter() {
                if *c >= 'A' as u8 && *c <= 'Z' as u8
                    || *c == '_' as u8
                    || *c == '.' as u8
                    || *c == '-' as u8
                    || *c >= '1' as u8 && *c <= '9' as u8
                {
                    name.push(*c as char);
                }
            }
            let properties = Properties::deserialize(&mut deserializer)?;

            if properties.kind & ENTRY_DIR == 0 {
                entries.insert(name, properties);
            }
        }
        Ok(Vdfs {
            deserializer: UnsafeCell::new(deserializer),
            entries,
            header,
        })
    }
    /// Get raw data of a specified entry
    fn get_raw_data(&self, properties: &Properties) -> Result<Vec<u8>> {
        let deserializer = unsafe { self.deserializer.get().as_mut().unwrap() };
        deserializer.seek(SeekFrom::Start(properties.offset as u64))?;
        let mut data_buf = vec![0_u8; properties.size as usize];
        deserializer.read(&mut data_buf)?;
        Ok(data_buf)
    }
    /// Get an entry, the name specified can be a subset of the real name
    pub fn get_by_name_slice(&self, name: &str) -> Result<Option<Entry>> {
        let key = match self.entries.keys().find(|k| k.contains(name)) {
            Some(k) => k,
            None => return Ok(None),
        };
        self.get_by_name(key)
    }
    /// Get an entry by the exact name that was given as input
    pub fn get_by_name(&self, name: &str) -> Result<Option<Entry>> {
        let properties = match self.entries.get(name) {
            Some(p) => p,
            None => return Err(Error::EntryNotFound),
        };
        let data = self.get_raw_data(properties)?;
        Ok(Some(Entry::new(name, properties, data)))
    }
    /// Lists all vdfs entries and some generic information
    pub fn list(&self) {
        for (name, prop) in &self.entries {
            println!(
                "Name: {}, Offset: {:x}, Size: {:x}, Kind: {:x}, Attr: {:x}",
                name, prop.offset, prop.size, prop.kind, prop.attr
            );
        }
        println!(
            "VDFS Archive, Size: {:x}, Time: {:x}, Offset: {:x}, Number Files: {}",
            self.header.data_size, self.header.timestamp, self.header.offset, self.header.num_files
        );
    }
}
