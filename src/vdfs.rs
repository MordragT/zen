use crate::FromReader;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::str;

const SIGNATURE_G1: &str = "PSVDSC_V2.00\r\n\r\n";
const SIGNATURE_G2: &str = "PSVDSC_V2.00\n\r\n\r";

const COMMENT_LENGTH: u64 = 256;
const SIGNATURE_LENGTH: usize = 16;
const ENTRY_NAME_LENGTH: usize = 64;
const ENTRY_DIR: u32 = 0x80000000;

#[derive(Debug)]
struct Properties {
    name: String,
    offset: u64,
    size: u32,
    kind: u32,
    attr: u32,
}
impl Properties {
    fn new(name: String, offset: u64, size: u32, kind: u32, attr: u32) -> Self {
        Self {
            name,
            offset,
            size,
            kind,
            attr,
        }
    }
}
#[derive(Debug)]
pub struct Entry<'a> {
    properties: &'a Properties,
    pub data: Vec<u8>,
}

impl<'a> Entry<'a> {
    fn new(properties: &'a Properties, data: Vec<u8>) -> Self {
        Self { properties, data }
    }
}

pub struct Vdfs<R: Read + Seek> {
    reader: UnsafeCell<R>,
    entries_properties: Vec<Properties>,
    offset: u64,
    data_size: u32,
    timestamp: u32,
    num_files: u32,
    names_map: HashMap<String, usize>,
}

impl<R: Read + Seek> Vdfs<R> {
    /// Creates a new Vdfs struct that holds the data of all entries
    pub fn new<'a>(mut reader: R) -> Result<Vdfs<R>, &'a str> {
        reader.seek(SeekFrom::Start(COMMENT_LENGTH)).unwrap();

        let mut signature_buf = [0_u8; SIGNATURE_LENGTH];
        reader.read_exact(&mut signature_buf).unwrap();
        let signature = str::from_utf8(&signature_buf).unwrap();
        if signature != SIGNATURE_G1 && signature != SIGNATURE_G2 {
            return Err("Signature of file does not match Gothic 1 or 2.");
        }

        let count = u32::from_reader(&mut reader);
        let num_files = u32::from_reader(&mut reader);
        let timestamp = u32::from_reader(&mut reader);
        let data_size = u32::from_reader(&mut reader);
        let root_cat_offset = u32::from_reader(&mut reader) as u64;
        let version = u32::from_reader(&mut reader);

        if version != 0x50 {
            return Err("Vdfs Version is not supported.");
        }

        reader.seek(SeekFrom::Start(root_cat_offset)).unwrap();

        let mut entries_properties = vec![];
        let mut names_map = HashMap::new();

        for _ in 0..count {
            let mut name_buf = [0_u8; ENTRY_NAME_LENGTH];
            reader.read_exact(&mut name_buf).unwrap();
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
            let offset = u32::from_reader(&mut reader) as u64;
            let size = u32::from_reader(&mut reader);
            let kind = u32::from_reader(&mut reader);
            let attr = u32::from_reader(&mut reader);

            if kind & ENTRY_DIR == 0 {
                let properties = Properties::new(name.clone(), offset, size, kind, attr);
                entries_properties.push(properties);
                names_map.insert(name, entries_properties.len() - 1);
            }
        }
        Ok(Vdfs {
            reader: UnsafeCell::new(reader),
            entries_properties,
            offset: root_cat_offset,
            data_size,
            timestamp,
            num_files,
            names_map,
        })
    }
    /// Get raw data of a specified entry
    fn get_raw_data(&self, properties: &Properties) -> Vec<u8> {
        let reader = unsafe { self.reader.get().as_mut().unwrap() };
        reader.seek(SeekFrom::Start(properties.offset)).unwrap();
        let mut data_buf = vec![0_u8; properties.size as usize];
        reader.read(&mut data_buf).unwrap();
        data_buf
    }
    /// Get an entry, the name specified can be a subset of the real name
    pub fn get_by_name_slice(&self, name: &str) -> Option<Entry> {
        let key = match self.names_map.keys().find(|k| k.contains(name)) {
            Some(k) => k,
            None => return None,
        };
        let index = self.names_map.get(key).unwrap();
        match self.entries_properties.get(*index) {
            Some(prop) => {
                let data = self.get_raw_data(prop);
                Some(Entry::new(prop, data))
            }
            None => {
                eprintln!("INTERNAL ERROR: names_map does not point to entry.");
                None
            }
        }
    }
    /// Get an entry by the exact name that was given as input
    pub fn get_by_name(&self, name: &str) -> Option<Entry> {
        let index = match self.names_map.get(name) {
            Some(i) => i,
            None => return None,
        };
        match self.entries_properties.get(*index) {
            Some(prop) => {
                let data = self.get_raw_data(prop);
                Some(Entry::new(prop, data))
            }
            None => {
                eprintln!("INTERNAL ERROR: names_map does not point to entry.");
                None
            }
        }
    }
    /// Lists all vdfs entries and some generic information
    pub fn list(&self) {
        for prop in &self.entries_properties {
            println!(
                "Name: {}, Offset: {:x}, Size: {:x}, Kind: {:x}, Attr: {:x}",
                prop.name, prop.offset, prop.size, prop.kind, prop.attr
            );
        }
        println!(
            "VDFS Archive, Size: {:x}, Time: {:x}, Offset: {:x}, Number Files: {}",
            self.data_size, self.timestamp, self.offset, self.num_files
        );
    }
}
