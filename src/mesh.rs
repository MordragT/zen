use crate::deserializer::{self, Chunk, Date, Header};
use crate::material::{self, AdvancedMaterial, BasicMaterial, Material};
use crate::math::{Float3, Float4};
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

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Float3>,
    indices: Vec<u32>,
    materials: Vec<Material>,
}

impl Mesh {
    pub fn new(data: &[u8]) -> Result<Mesh, &str> {
        let mut reader = Cursor::new(data);

        // min, max
        let mut bounding_box: (Float3, Float3);
        loop {
            let chunk =
                bincode::deserialize_from::<&mut Cursor<&[u8]>, Chunk>(&mut reader).unwrap();
            let chunk_end = SeekFrom::Current(chunk.length as i64);
            let mut materials = vec![];
            match chunk.id {
                MESH => {
                    let version =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, u32>(&mut reader).unwrap();
                    let date =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, Date>(&mut reader).unwrap();
                    let name = bincode::deserialize_from::<&mut Cursor<&[u8]>, String>(&mut reader)
                        .unwrap();
                    println!(
                        "Reading mesh {} with version: {}, Timestamp: {}",
                        name, version, date
                    );
                    reader.seek(chunk_end).unwrap();
                }
                BBOX3D => {
                    let min = bincode::deserialize_from::<&mut Cursor<&[u8]>, Float4>(&mut reader)
                        .unwrap();
                    let max = bincode::deserialize_from::<&mut Cursor<&[u8]>, Float4>(&mut reader)
                        .unwrap();
                    bounding_box = (Float3(min.0, min.1, min.2), Float3(max.0, max.1, max.2));
                    reader.seek(chunk_end).unwrap();
                }
                MATLIST => {
                    //let _header =
                    // Skip name and chunk headers
                    let _material_name =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, String>(&mut reader)
                            .unwrap();
                    let _chunk_size =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, u32>(&mut reader).unwrap();
                    let version =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, u16>(&mut reader).unwrap();
                    let _object_index =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, u32>(&mut reader).unwrap();
                    deserializer::skip_spaces(&mut reader);

                    // Skip chunk header
                    let _name =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, String>(&mut reader)
                            .unwrap();
                    let _class_name =
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, String>(&mut reader)
                            .unwrap();

                    // Save into Vec
                    let material: Material = if version == material::GOTHIC1 {
                        bincode::deserialize_from::<&mut Cursor<&[u8]>, BasicMaterial>(&mut reader)
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
                _ => {
                    reader.seek(chunk_end).unwrap();
                }
            }
        }
    }
}
