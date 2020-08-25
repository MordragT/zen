use crate::math::Float3;
use crate::{Chunk, Date};
use crate::{FromBufReader, FromReader};
use serde::Deserialize;
use std::io::Cursor;

const MESH: u16 = 0xB000;
const BBOX3D: u16 = 0xB010;
const MATLIST: u16 = 0xB020;
const LIGHTMAPLIST: u16 = 0xB025;
const LIGHTMAPLIST_SHARED: u16 = 0xB026;
const VERTLIST: u16 = 0xB030;
const FEATLIST: u16 = 0xB040;
const POLYLIST: u16 = 0xB050;
const MESH_END: u16 = 0xB060;

#[derive(Deserialize, Debug)]
pub struct Mesh {
    vertices: Vec<Float3>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new(data: &[u8]) -> Result<Mesh, &str> {
        let mut parser = Cursor::new(data);
        loop {
            let chunk = Chunk::from_reader(&mut parser);
            match chunk.id {
                MESH => {
                    let version = u32::from_reader(&mut parser);
                    let date = Date::from_reader(&mut parser);
                    let name = String::from_buf_reader(&mut parser);
                    println!(
                        "Reading mesh {} with version: {}, Timestamp: {}",
                        name, version, date
                    );
                }
                BBOX3D => {}
                _ => (),
            }
        }
    }
}
