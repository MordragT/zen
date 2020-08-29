use serde::Deserialize;
use std::io::{Cursor, Seek, SeekFrom};
use structs::*;
use zen_material::{self, AdvancedMaterial, BasicMaterial, Material};
use zen_parser::prelude::AsciiDeserializer;

pub mod structs;

const MESH: u16 = 0xB000;
const BBOX3D: u16 = 0xB010;
const MATLIST: u16 = 0xB020;
const LIGHTMAPLIST: u16 = 0xB025;
const LIGHTMAPLIST_SHARED: u16 = 0xB026;
const VERTLIST: u16 = 0xB030;
const FEATLIST: u16 = 0xB040;
const POLYLIST: u16 = 0xB050;
const MESH_END: u16 = 0xB060;

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<(f32, f32, f32)>,
    indices: Vec<u32>,
    materials: Vec<Material>,
}

impl Mesh {
    pub fn new(data: &[u8]) -> Result<Mesh, &str> {
        let mut reader = Cursor::new(data);

        // min, max
        let mut bounding_box: ((f32, f32, f32), (f32, f32, f32));
        loop {
            let chunk =
                bincode::deserialize_from::<&mut Cursor<&[u8]>, Chunk>(&mut reader).unwrap();
            let chunk_end = SeekFrom::Current(chunk.length as i64);
            let mut materials = vec![];
            match chunk.id {
                MESH => {
                    #[derive(Deserialize)]
                    struct Info {
                        version: u32,
                        date: Date,
                        name: String,
                    }
                    let info =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, Info>(&mut reader).unwrap();
                    println!(
                        "Reading mesh {} with version: {}, Timestamp: {}",
                        info.name, info.version, info.date
                    );
                    reader.seek(chunk_end).unwrap();
                }
                BBOX3D => {
                    let (min, max) = bincode::deserialize_from::<
                        &mut Cursor<&[u8]>,
                        ((f32, f32, f32, f32), (f32, f32, f32, f32)),
                    >(&mut reader)
                    .unwrap();
                    bounding_box = ((min.0, min.1, min.2), (max.0, max.1, max.2));
                    reader.seek(chunk_end).unwrap();
                }
                MATLIST => {
                    let mut ascii_deserializer = AsciiDeserializer::from_cursor(reader);
                    ascii_deserializer.read_header().unwrap();
                    reader = ascii_deserializer.into_cursor();

                    let material_num =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, u32>(&mut reader).unwrap();

                    for _ in 0..material_num {
                        #[derive(Deserialize)]
                        struct MaterialHeader {
                            material_name: String,
                            chunk_size: u32,
                            version: u16,
                            object_index: u32,
                        }
                        // Skip name and chunk headers
                        let material = bincode::deserialize_from::<
                            &mut Cursor<&[u8]>,
                            MaterialHeader,
                        >(&mut reader)
                        .unwrap();

                        zen_parser::skip_spaces(&mut reader);

                        // Skip chunk header
                        let _name =
                            bincode::deserialize_from::<&mut Cursor<&[u8]>, String>(&mut reader)
                                .unwrap();
                        let _class_name =
                            bincode::deserialize_from::<&mut Cursor<&[u8]>, String>(&mut reader)
                                .unwrap();

                        // Save into Vec
                        let material: Material = if material.version == zen_material::GOTHIC1 {
                            bincode::deserialize_from::<&mut Cursor<&[u8]>, BasicMaterial>(
                                &mut reader,
                            )
                            .unwrap()
                            .into()
                        } else {
                            bincode::deserialize_from::<&mut Cursor<&[u8]>, AdvancedMaterial>(
                                &mut reader,
                            )
                            .unwrap()
                            .into()
                        };
                        materials.push(material);
                    }
                }
                _ => {
                    reader.seek(chunk_end).unwrap();
                }
            }
        }
    }
}
