//! This crate can deserialize [.msh](Msh) (compiled meshes),
//! and convert them into [Model] objects.
//!
//! ```rust
//! use std::{convert::TryFrom, fs::File, io::Cursor};
//! use zen_archive::Vdfs;
//! use zen_msh::Msh;
//! use zen_model::{gltf, Model};
//! use zen_types::path::INSTANCE;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let vdf_file = File::open(INSTANCE.meshes())?;
//! let vdf = Vdfs::new(vdf_file)?;
//! let mesh_entry = vdf
//!     .get_by_name("MFX_FEAR4.MSH")
//!     .expect("Should be there!");
//! let cursor = Cursor::new(mesh_entry.data);
//! let mesh = Msh::new(cursor, "FEAR4")?;
//! let model = Model::try_from(mesh)?;
//! let _gltf = gltf::to_gltf(mesh, gltf::Output::Binary);
//! #    Ok(())
//! # }
//!

use crate::{material::*, math::Vec3};
pub use error::MshError;
use error::MshResult;
use serde::Deserialize;
use std::convert::TryFrom;
use std::io::{Seek, SeekFrom};
use zen_parser::prelude::*;

mod error;
mod structures;

const MESH: u16 = 0xB000;
const BBOX3D: u16 = 0xB010;
const MAT_LIST: u16 = 0xB020;
const LIGHT_MAP_LIST: u16 = 0xB025;
const LIGHT_MAP_LIST_SHARED: u16 = 0xB026;
const VERT_LIST: u16 = 0xB030;
const FEAT_LIST: u16 = 0xB040;
const POLY_LIST: u16 = 0xB050;
const MESH_END: u16 = 0xB060;

const GOTHIC2_6: u32 = 265;
const GOTHIC1_08K: u32 = 9;

// pub struct MshLoader;

// impl AssetLoader for MshLoader {
//     type Error = Error;
//     fn load(data: &[u8], name: &str) -> Result<Asset> {
//         let cursor = Cursor::new(data);
//         let msh = Msh::new(cursor, name)?;
//         let model = Model::try_from(msh)?;
//         Ok(Asset::Model(model))
//     }
//     fn extensions() -> &'static [&'static str] {
//         &["msh"]
//     }
// }

pub struct Msh {
    pub name: String,
    pub materials: Vec<BasicMaterial>,
    pub vertices: Vec<Vec3<f32>>,
    pub features: Vec<structures::FeatureChunk>,
    pub polygons: Vec<structures::Polygon>,
}

impl TryFrom<Box<MshBuilder>> for Msh {
    type Error = MshError;
    fn try_from(builder: Box<MshBuilder>) -> MshResult<Self> {
        let materials = match builder.materials {
            Some(m) => m,
            None => return Err(MshError::ExpectedValue("Expected material vec.".to_owned())),
        };
        let vertices = match builder.vertices {
            Some(v) => v,
            None => return Err(MshError::ExpectedValue("Expected vertices vec.".to_owned())),
        };
        let features = match builder.features {
            Some(f) => f,
            None => return Err(MshError::ExpectedValue("Expected features vec.".to_owned())),
        };
        let polygons = match builder.polygons {
            Some(p) => p,
            None => return Err(MshError::ExpectedValue("Expected polygons vec.".to_owned())),
        };
        Ok(Self {
            name: builder.name,
            materials,
            vertices,
            features,
            polygons,
        })
    }
}

#[derive(Default)]
pub struct MshBuilder {
    pub name: String,
    pub version: Option<u32>,
    //pub mesh: Option<()>,
    //pub bbox3d: Option<()>,
    pub materials: Option<Vec<BasicMaterial>>,
    pub light_maps: Option<()>,
    pub vertices: Option<Vec<Vec3<f32>>>,
    pub features: Option<Vec<structures::FeatureChunk>>,
    pub polygons: Option<Vec<structures::Polygon>>,
}

fn deserialize_version<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: u64,
) -> MshResult<u32> {
    let mut deserializer = BinaryDeserializer::from(reader);
    #[derive(Deserialize)]
    struct Info {
        version: u32,
        _date: structures::Date,
        _name: String,
    }

    let info = Info::deserialize(&mut deserializer)?;
    deserializer.seek(SeekFrom::Start(chunk_end))?;
    Ok(info.version)
}

fn deserialize_bbox3d<R: BinaryRead + AsciiRead>(reader: &mut R, chunk_end: u64) -> MshResult<()> {
    let mut deserializer = BinaryDeserializer::from(reader);
    let (_min, _max) =
        <((f32, f32, f32, f32), (f32, f32, f32, f32))>::deserialize(&mut deserializer)?;
    //bounding_box = ((min.0, min.1, min.2), (max.0, max.1, max.2));
    deserializer.seek(SeekFrom::Start(chunk_end))?;
    Ok(())
}

fn deserialize_materials<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: u64,
) -> MshResult<Vec<BasicMaterial>> {
    let mut deserializer = BinaryDeserializer::from(reader);
    let _header = Reader::from(&mut deserializer.parser).read_header()?;

    let material_num = u32::deserialize(&mut deserializer)?;
    let materials = (0..material_num)
        .into_iter()
        .map(|_| {
            let _name = String::deserialize(&mut deserializer)?;
            // Skip name and chunk headers
            let material_header = ChunkHeader::deserialize(&mut deserializer)?;

            // Skip chunk header
            let _name = String::deserialize(&mut deserializer)?;
            let _class_name = String::deserialize(&mut deserializer)?;

            // Save into Vec
            match material_header.version {
                GOTHIC2 => Ok(BasicMaterial::deserialize(&mut deserializer)?),
                _ => Ok(AdvancedMaterial::deserialize(&mut deserializer)?.into()),
            }
        })
        .collect::<MshResult<Vec<BasicMaterial>>>()?;
    deserializer.seek(SeekFrom::Start(chunk_end))?;
    Ok(materials)
}

fn deserialize_light_maps<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: u64,
) -> MshResult<()> {
    let mut deserializer = BinaryDeserializer::from(reader);
    deserializer.seek(SeekFrom::Start(chunk_end))?;
    Ok(())
}

fn deserialize_vertices<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: u64,
) -> MshResult<Vec<Vec3<f32>>> {
    let mut deserializer = BinaryDeserializer::from(reader);
    let num_vertices = u32::deserialize(&mut deserializer)?;
    deserializer.len_queue.push(num_vertices as usize);
    //vertices = <Vec<(f32, f32, f32)>>::deserialize(&mut deserializer)?;
    let vertices = <Vec<Vec3<f32>>>::deserialize(&mut deserializer)?;
    deserializer.seek(SeekFrom::Start(chunk_end))?;
    Ok(vertices)
}

fn deserialize_features<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: u64,
) -> MshResult<Vec<structures::FeatureChunk>> {
    let mut deserializer = BinaryDeserializer::from(reader);
    let num_feats = u32::deserialize(&mut deserializer)?;
    deserializer.len_queue.push(num_feats as usize);
    let features = <Vec<structures::FeatureChunk>>::deserialize(&mut deserializer)?;
    deserializer.seek(SeekFrom::Start(chunk_end))?;
    Ok(features)
}

fn deserialize_poly_list<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: u64,
    version: u32,
) -> MshResult<Vec<structures::Polygon>> {
    let mut deserializer = BinaryDeserializer::from(reader);
    let num_polys = u32::deserialize(&mut deserializer)?;
    let polygons = (0..num_polys)
        .into_iter()
        .map(|_| {
            // let data_block_seed = deserializer.seek(SeekFrom::Current(0))?;
            // deserializer.seek(chunk_end);

            // TODO: nochmal in referenz gucken, deserialzation geschieht erst nachher

            #[repr(packed(1))]
            #[derive(Deserialize)]
            struct PolygonData {
                pub material_index: i16,
                pub light_map_index: i16,
                pub plane: structures::PlanePacked,
            }
            let polygon_data = PolygonData::deserialize(&mut deserializer)?;

            let flags: structures::PolyFlags = match version {
                GOTHIC2_6 => {
                    <structures::PolyGothicTwoFlags>::deserialize(&mut deserializer)?.into()
                }
                GOTHIC1_08K => todo!(),
                _ => return Err(MshError::UnknownGameVersion(version)),
            };

            let num_indices = u8::deserialize(&mut deserializer)?;

            let indices = (0..num_indices)
                .map(|_| {
                    let index: structures::Index = match version {
                        GOTHIC2_6 => {
                            <structures::IndexPacked<u32>>::deserialize(&mut deserializer)?.into()
                        }
                        GOTHIC1_08K => {
                            <structures::IndexPacked<u16>>::deserialize(&mut deserializer)?.into()
                        }
                        _ => return Err(MshError::UnknownGameVersion(version)),
                    };
                    return Ok(index);
                })
                .collect::<MshResult<Vec<structures::Index>>>()?;

            Ok(structures::Polygon::new(
                polygon_data.material_index,
                polygon_data.light_map_index,
                polygon_data.plane.into(),
                flags,
                num_indices,
                indices,
            ))
        })
        .collect::<MshResult<Vec<structures::Polygon>>>()?;
    deserializer.seek(SeekFrom::Start(chunk_end))?;
    Ok(polygons)
}

fn read_chunk<R: BinaryRead + AsciiRead>(
    mut reader: R,
    mut builder: Box<MshBuilder>,
) -> MshResult<Msh> {
    #[derive(Deserialize)]
    #[repr(C, packed(4))]
    struct Chunk {
        id: u16,
        length: u32,
    }

    let mut deserializer = BinaryDeserializer::from(&mut reader);
    let chunk = <Chunk>::deserialize(&mut deserializer)?;
    let chunk_end = deserializer.seek(SeekFrom::Current(0))? + chunk.length as u64;

    match chunk.id {
        MESH => {
            builder.version = Some(deserialize_version::<R>(&mut reader, chunk_end)?);
            read_chunk(reader, builder)
        }
        BBOX3D => {
            let _bbox3d = deserialize_bbox3d::<R>(&mut reader, chunk_end)?;
            read_chunk(reader, builder)
        }
        MAT_LIST => {
            builder.materials = Some(deserialize_materials::<R>(&mut reader, chunk_end)?);
            read_chunk(reader, builder)
        }
        LIGHT_MAP_LIST => {
            builder.light_maps = Some(deserialize_light_maps::<R>(&mut reader, chunk_end)?);
            read_chunk(reader, builder)
        }
        //skip
        LIGHT_MAP_LIST_SHARED => read_chunk(reader, builder),
        VERT_LIST => {
            builder.vertices = Some(deserialize_vertices::<R>(&mut reader, chunk_end)?);
            read_chunk(reader, builder)
        }
        FEAT_LIST => {
            builder.features = Some(deserialize_features::<R>(&mut reader, chunk_end)?);
            read_chunk(reader, builder)
        }
        POLY_LIST => {
            let version = match builder.version {
                Some(v) => v,
                None => {
                    return Err(MshError::ExpectedValue(
                        "Expected present version value.".to_owned(),
                    ))
                }
            };
            builder.polygons = Some(deserialize_poly_list::<R>(&mut reader, chunk_end, version)?);
            read_chunk(reader, builder)
        }
        MESH_END => Msh::try_from(builder),
        _ => {
            eprintln!("Unknown chunk.");
            deserializer.seek(SeekFrom::Start(chunk_end))?;
            read_chunk(reader, builder)
        }
    }
}

impl Msh {
    pub fn new<R: BinaryRead + AsciiRead>(reader: R, name: &str) -> MshResult<Msh> {
        read_chunk(
            reader,
            Box::new(MshBuilder {
                name: name.to_owned(),
                ..Default::default()
            }),
        )
    }
}
