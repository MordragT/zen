use crate::math::{Float3, Float4};
use crate::{Chunk, Date};
use crate::{FromBufReader, FromReader};
use serde::Deserialize;
use std::io::{Cursor, Seek, SeekFrom};

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
        let mut reader = Cursor::new(data);

        // min, max
        let mut bounding_box: (Float3, Float3);
        loop {
            let chunk = Chunk::from_reader(&mut reader);
            let chunk_end = SeekFrom::Current(chunk.length as i64);
            match chunk.id {
                MESH => {
                    let version = u32::from_reader(&mut reader);
                    let date = Date::from_reader(&mut reader);
                    let name = String::from_buf_reader(&mut reader);
                    println!(
                        "Reading mesh {} with version: {}, Timestamp: {}",
                        name, version, date
                    );
                    reader.seek(chunk_end).unwrap();
                }
                BBOX3D => {
                    let min = Float4::from_reader(&mut reader);
                    let max = Float4::from_reader(&mut reader);
                    bounding_box = (Float3(min.0, min.1, min.2), Float3(max.0, max.1, max.2));
                    reader.seek(chunk_end).unwrap();
                }
                MATLIST => {
                    // Skip name and chunk headers
                    let _material_name = String::from_buf_reader(&mut reader);
                    let _chunk_size = u32::from_reader(&mut reader);
                    let version = u16::from_reader(&mut reader);
                    let _object_index = u32::from_reader(&mut reader);
                    crate::skip_spaces(&mut reader);

                    // Skip chunk header
                    let _name = String::from_buf_reader(&mut reader);
                    let _class_name = String::from_buf_reader(&mut reader);

                    // Save into Vec
                }
                _ => {
                    reader.seek(chunk_end).unwrap();
                }
            }
        }
    }
}
