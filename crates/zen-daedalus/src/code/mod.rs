use crate::data::Data;
use error::*;
use label::{Flag, Kind, Label, Properties};
use serde::Deserialize;
use std::{fs::File, io::Seek};
use zen_parser::prelude::*;

pub mod error;
pub mod label;
pub struct Code<R: BinaryRead> {
    deserializer: BinaryDeserializer<R>,
    data: Vec<Data>,
    labels: Vec<(Label, usize)>,
    len: usize,
}

impl<R: BinaryRead> Code<R> {
    pub fn open(reader: R) -> Result<Self> {
        let mut deserializer = BinaryDeserializer::from(reader);

        let version = u8::deserialize(&mut deserializer)?;
        let label_count = u32::deserialize(&mut deserializer)?;

        let addresses = (0..label_count)
            .into_iter()
            .map(|_| {
                let address = u32::deserialize(&mut deserializer)? as usize;
                Ok(address)
            })
            .collect::<Result<Vec<usize>>>()?;

        let labels = (0..label_count)
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
                if !properties.has_flag(Flag::ClassVar) {
                    match properties.get_kind() {
                        Kind::Float => todo!(),
                        Kind::Int => todo!(),
                        Kind::CharString => todo!(),
                        Kind::Class => {
                            let offset = i32::deserialize(&mut deserializer)?;
                        }
                        Kind::Func | Kind::Prototype | Kind::Instance => {
                            let address = i32::deserialize(&mut deserializer)?;
                        }
                        Kind::Void => (),
                    }
                }

                let parent = i32::deserialize(&mut deserializer)?;

                // let mut buf = [0_u8; 182];
                // deserializer.parser.read(&mut buf);
                // buf.into_iter().for_each(|u| {
                //     if *u < 128 && *u > 33 {
                //         print!("{} ", *u as char)
                //     } else {
                //         print!("0x{:X} ", *u)
                //     }
                // });

                // todo!();

                Ok(Label { name, parent })
            })
            .collect::<Result<Vec<Label>>>()?;

        addresses
            .iter()
            .zip(labels.iter())
            .for_each(|(address, label)| println!("{} -> {}", label.name, address));

        todo!()
    }
    pub fn len(&self) -> usize {
        self.len
    }
}
