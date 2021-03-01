use error::*;
use serde::Deserialize;
use std::{cell::UnsafeCell, collections::HashMap, io::prelude::*, io::SeekFrom};
use zen_parser::prelude::*;

pub mod error;

const SIGNATURE_G1: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\r\n\r\n";
const SIGNATURE_G2: [u8; SIGNATURE_LENGTH] = *b"PSVDSC_V2.00\n\r\n\r";

const COMMENT_LENGTH: u64 = 256;
const SIGNATURE_LENGTH: usize = 16;
const ENTRY_NAME_LENGTH: usize = 64;
const ENTRY_DIR: u32 = 0x80000000;

#[derive(Deserialize, Debug)]
/// Entry Properties
struct Properties {
    offset: u32,
    size: u32,
    kind: u32,
    attr: u32,
}
#[derive(Debug)]
/// Vdfs Entry
pub struct Entry<'a> {
    pub name: String,
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
/// Vdfs Header attributes
struct Header {
    signature: [u8; SIGNATURE_LENGTH],
    count: u32,
    num_files: u32,
    timestamp: u32,
    data_size: u32,
    offset: u32,
    version: u32,
}
/// Vdfs reader
pub struct Vdfs<R: BinaryRead> {
    deserializer: UnsafeCell<BinaryDeserializer<R>>,
    header: Header,
    entries: HashMap<String, Properties>,
}

fn print_entry(name: &str, properties: &Properties) {
    println!(
        "Name: {}, Offset: {:x}, Size: {:x}, Kind: {:x}, Attr: {:x}",
        name, properties.offset, properties.size, properties.kind, properties.attr
    );
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

        deserializer
            .parser
            .seek(SeekFrom::Start(header.offset as u64))?;

        let entries = (0..header.count)
            .map(|_| -> Result<(String, Properties)> {
                let mut name_buf = [0_u8; ENTRY_NAME_LENGTH];
                deserializer.parser.read_exact(&mut name_buf)?;
                let name = name_buf
                    .iter()
                    .filter_map(|c| {
                        if *c >= 'A' as u8 && *c <= 'Z' as u8
                            || *c == '_' as u8
                            || *c == '.' as u8
                            || *c == '-' as u8
                            || *c >= '0' as u8 && *c <= '9' as u8
                        {
                            Some(*c as char)
                        } else {
                            None
                        }
                    })
                    .collect::<String>();
                Ok((name, Properties::deserialize(&mut deserializer)?))
            })
            .filter(|res| match res {
                Ok((_, properties)) => properties.kind & ENTRY_DIR == 0,
                Err(_) => true,
            })
            .collect::<Result<HashMap<String, Properties>>>()?;

        Ok(Vdfs {
            deserializer: UnsafeCell::new(deserializer),
            entries,
            header,
        })
    }
    /// Get raw data of a specified entry
    fn get_raw_data(&self, properties: &Properties) -> Result<Vec<u8>> {
        let deserializer = unsafe {
            self.deserializer
                .get()
                .as_mut()
                .expect("Deserializer should be available.")
        };
        deserializer
            .parser
            .seek(SeekFrom::Start(properties.offset as u64))?;
        let mut data_buf = vec![0_u8; properties.size as usize];
        deserializer.parser.read(&mut data_buf)?;
        Ok(data_buf)
    }
    /// Get an entry, the name specified can be a subset of the real name
    pub fn get_by_name_slice(&self, name: &str) -> Option<Entry> {
        let key = match self.entries.keys().find(|k| k.contains(name)) {
            Some(k) => k,
            None => return None,
        };
        self.get_by_name(key)
    }
    /// Get an entry by the exact name that was given as input
    pub fn get_by_name(&self, name: &str) -> Option<Entry> {
        let properties = match self.entries.get(name) {
            Some(p) => p,
            None => return None,
        };
        let data = self
            .get_raw_data(properties)
            .expect("Should always find specified properties.");
        Some(Entry::new(name, properties, data))
    }
    /// Lists all vdfs entries and some generic information
    pub fn list(&self) {
        self.entries
            .iter()
            .for_each(|(name, prop)| print_entry(name, prop));
        self.print_archive_information();
    }
    pub fn filter_list(&self, input: &str) {
        self.entries
            .iter()
            .filter(|(name, _)| name.contains(input))
            .for_each(|(name, prop)| print_entry(name, prop));
        self.print_archive_information();
    }
    fn print_archive_information(&self) {
        println!(
            "VDFS Archive, Size: {:x}, Time: {:x}, Offset: {:x}, Number Files: {}",
            self.header.data_size, self.header.timestamp, self.header.offset, self.header.num_files
        );
    }
}
