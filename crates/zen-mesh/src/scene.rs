use crate::error::*;
use serde::Deserialize;
use std::io::{Seek, SeekFrom};
use zen_parser::prelude::*;
use zen_types::mesh::scene;

const MESH: u16 = 0xB000;
const BBOX3D: u16 = 0xB010;
const MAT_LIST: u16 = 0xB020;
const LIGHT_MAP_LIST: u16 = 0xB025;
const LIGHT_MAP_LIST_SHARED: u16 = 0xB026;
const VERT_LIST: u16 = 0xB030;
const FEAT_LIST: u16 = 0xB040;
const POLY_LIST: u16 = 0xB050;
const MESH_END: u16 = 0xB060;

pub struct SceneMesh {
    polygons: Option<Vec<scene::Polygon>>,
}

// impl SceneMesh {
//     pub fn from_world<'a, R: BinaryRead + AsciiRead>(reader: R) -> Result<SceneMesh> {
//         let mut deserializer = BinaryDeserializer::from(reader);

//         // min, max
//         //let mut bounding_box: ((f32, f32, f32), (f32, f32, f32));
//         let mut name = String::new();
//         let mut vertices = vec![];
//         let mut _features = vec![];
//         let mut polygons = vec![];
//         let mut materials = vec![];
//         let mut version = 0;

//         loop {
//             let chunk = mesh::Chunk::deserialize(&mut deserializer)?;
//             let chunk_end = SeekFrom::Current(chunk.length as i64);
//             //println!("id:{} -> length: {}", chunk.id, chunk.length);
//             match chunk.id {
//                 MESH => {
//                     #[derive(Deserialize)]
//                     struct Info {
//                         version: u32,
//                         date: mesh::Date,
//                         name: String,
//                     }
//                     let info = Info::deserialize(&mut deserializer)?;
//                     println!("Reading mesh {} with version: {}", info.name, info.version);
//                     version = info.version;
//                     name = info.name;
//                     deserializer.seek(chunk_end)?;
//                 }
//                 BBOX3D => {
//                     println!("Reading bounding box");
//                     let (_min, _max) = <((f32, f32, f32, f32), (f32, f32, f32, f32))>::deserialize(
//                         &mut deserializer,
//                     )?;
//                     //bounding_box = ((min.0, min.1, min.2), (max.0, max.1, max.2));
//                     deserializer.seek(chunk_end)?;
//                 }
//                 MAT_LIST => {
//                     println!("Reading material list");
//                     let mut ascii_deserializer = AsciiDeserializer::from(deserializer);
//                     ascii_deserializer.read_header()?;
//                     deserializer = ascii_deserializer.into();

//                     let material_num = u32::deserialize(&mut deserializer)?;
//                     for _ in 0..material_num {
//                         let material: material::Material = {
//                             let _name = String::deserialize(&mut deserializer)?;
//                             // Skip name and chunk headers
//                             let material_header =
//                                 material::ChunkHeader::deserialize(&mut deserializer)?;

//                             // Skip chunk header
//                             let _name = String::deserialize(&mut deserializer)?;
//                             let _class_name = String::deserialize(&mut deserializer)?;

//                             // Save into Vec
//                             match material_header.version {
//                                 material::GOTHIC2 => {
//                                     material::BasicMaterial::deserialize(&mut deserializer)?.into()
//                                 }
//                                 _ => material::AdvancedMaterial::deserialize(&mut deserializer)?
//                                     .into(),
//                             }
//                         };
//                         materials.push(material);
//                     }
//                     deserializer.seek(chunk_end)?;
//                 }
//                 LIGHT_MAP_LIST => {
//                     println!("Reading light map list");
//                     deserializer.seek(chunk_end)?;
//                 }
//                 LIGHT_MAP_LIST_SHARED => {
//                     println!("Reading light map list");
//                     deserializer.seek(chunk_end)?;
//                 }
//                 VERT_LIST => {
//                     println!("Reading vertice list");
//                     let num_vertices = u32::deserialize(&mut deserializer)?;
//                     deserializer.len_queue.push(num_vertices as usize);
//                     vertices = <Vec<(f32, f32, f32)>>::deserialize(&mut deserializer)?;
//                 }
//                 FEAT_LIST => {
//                     println!("Reading feature list");
//                     let num_feats = u32::deserialize(&mut deserializer)?;
//                     deserializer.len_queue.push(num_feats as usize);
//                     _features = <Vec<mesh::FeatureChunk>>::deserialize(&mut deserializer)?;
//                 }
//                 POLY_LIST => {
//                     println!("Reading polygon list");
//                     let num_polys = u32::deserialize(&mut deserializer)?;
//                     for _ in 0..num_polys {
//                         // let data_block_seed = deserializer.seek(SeekFrom::Current(0))?;
//                         // deserializer.seek(chunk_end);

//                         // TODO: nochmal in referenz gucken, deserialzation geschieht erst nachher

//                         #[repr(packed(1))]
//                         #[derive(Deserialize)]
//                         struct PolygonData {
//                             pub material_index: i16,
//                             pub light_map_index: i16,
//                             pub plane: mesh::PlanePacked,
//                         }
//                         let polygon_data = PolygonData::deserialize(&mut deserializer)?;

//                         let flags: mesh::PolyFlags = match version {
//                             GOTHIC2_6 => {
//                                 <mesh::PolyGothicTwoFlags>::deserialize(&mut deserializer)?.into()
//                             }
//                             GOTHIC1_08K => todo!(),
//                             _ => return Err(Error::UnknownGameVersion),
//                         };

//                         let num_indices = u8::deserialize(&mut deserializer)?;

//                         let indices_result = (0..num_indices)
//                             .map(|_| {
//                                 let index: mesh::Index = match version {
//                                     GOTHIC2_6 => {
//                                         <mesh::IndexPacked<u32>>::deserialize(&mut deserializer)?
//                                             .into()
//                                     }
//                                     GOTHIC1_08K => {
//                                         <mesh::IndexPacked<u16>>::deserialize(&mut deserializer)?
//                                             .into()
//                                     }
//                                     _ => return Err(Error::UnknownGameVersion),
//                                 };
//                                 return Ok(index);
//                             })
//                             .collect::<Result<Vec<mesh::Index>>>();

//                         let indices = match indices_result {
//                             Ok(i) => i,
//                             Err(s) => return Err(s),
//                         };

//                         let polygon = mesh::Polygon::new(
//                             polygon_data.material_index,
//                             polygon_data.light_map_index,
//                             polygon_data.plane.into(),
//                             flags,
//                             num_indices,
//                             indices,
//                         );
//                         polygons.push(polygon);
//                     }
//                     deserializer.seek(chunk_end)?;
//                 }
//                 MESH_END => break,
//                 _ => {
//                     deserializer.seek(chunk_end)?;
//                 }
//             }
//         }
//         Ok(Self {
//             name,
//             vertices,
//             polygons: Some(polygons),
//             sub_meshes: None,
//         })
//     }
// }
