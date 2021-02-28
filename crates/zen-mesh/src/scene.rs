use crate::error::*;
use serde::Deserialize;
use std::io::{Seek, SeekFrom};
use vek::Vec3;
use zen_parser::prelude::*;
use zen_types::{
    material,
    mesh::{self, scene},
};

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
    pub name: String,
    polygons: Option<Vec<scene::Polygon>>,
}

impl From<Box<SceneMeshBuilder>> for SceneMesh {
    fn from(builder: Box<SceneMeshBuilder>) -> Self {
        todo!()
    }
}

#[derive(Default)]
pub struct SceneMeshBuilder {
    pub version: Option<u32>,
    pub mesh: Option<()>,
    pub bbox3d: Option<()>,
    pub mat_list: Option<()>,
    pub light_map_list: Option<()>,
    pub vert_list: Option<()>,
    pub feat_list: Option<()>,
    pub poly_list: Option<()>,
}

fn deserialize_mesh<R: BinaryRead + AsciiRead>(reader: &mut R, chunk_end: SeekFrom) -> Result<u32> {
    let mut deserializer = BinaryDeserializer::from(reader);
    #[derive(Deserialize)]
    struct Info {
        version: u32,
        date: scene::Date,
        name: String,
    }
    let info = Info::deserialize(&mut deserializer)?;
    println!("Reading mesh {} with version: {}", info.name, info.version);
    deserializer.seek(chunk_end)?;
    Ok(info.version)
}

fn deserialize_bbox3d<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<()> {
    let mut deserializer = BinaryDeserializer::from(reader);
    println!("Reading bounding box");
    let (_min, _max) =
        <((f32, f32, f32, f32), (f32, f32, f32, f32))>::deserialize(&mut deserializer)?;
    //bounding_box = ((min.0, min.1, min.2), (max.0, max.1, max.2));
    deserializer.seek(chunk_end)?;
    Ok(())
}

fn deserialize_mat_list<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<Vec<material::GeneralMaterial>> {
    println!("Reading material list");
    let mut ascii_deserializer = AsciiDeserializer::from(reader);
    ascii_deserializer.read_header()?;
    let reader = ascii_deserializer.parser;
    let mut deserializer = BinaryDeserializer::from(reader);

    let material_num = u32::deserialize(&mut deserializer)?;
    let materials = (0..material_num)
        .into_iter()
        .map(|_| {
            let _name = String::deserialize(&mut deserializer)?;
            // Skip name and chunk headers
            let material_header = material::ChunkHeader::deserialize(&mut deserializer)?;

            // Skip chunk header
            let _name = String::deserialize(&mut deserializer)?;
            let _class_name = String::deserialize(&mut deserializer)?;

            // Save into Vec
            match material_header.version {
                material::GOTHIC2 => {
                    Ok(material::BasicMaterial::deserialize(&mut deserializer)?.into())
                }
                _ => Ok(material::AdvancedMaterial::deserialize(&mut deserializer)?.into()),
            }
        })
        .collect::<Result<Vec<material::GeneralMaterial>>>()?;
    deserializer.seek(chunk_end)?;
    Ok(materials)
}

fn deserialize_light_mat_list<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<()> {
    let mut deserializer = BinaryDeserializer::from(reader);
    deserializer.seek(chunk_end)?;
    Ok(())
}

fn deserialize_vert_list<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<Vec<Vec3<f32>>> {
    let mut deserializer = BinaryDeserializer::from(reader);
    println!("Reading vertice list");
    let num_vertices = u32::deserialize(&mut deserializer)?;
    deserializer.len_queue.push(num_vertices as usize);
    //vertices = <Vec<(f32, f32, f32)>>::deserialize(&mut deserializer)?;
    let vertices = <Vec<Vec3<f32>>>::deserialize(&mut deserializer)?;
    deserializer.seek(chunk_end)?;
    Ok(vertices)
}

fn deserialize_feat_list<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<()> {
    let mut deserializer = BinaryDeserializer::from(reader);
    println!("Reading feature list");
    let num_feats = u32::deserialize(&mut deserializer)?;
    deserializer.len_queue.push(num_feats as usize);
    let _features = <Vec<scene::FeatureChunk>>::deserialize(&mut deserializer)?;
    deserializer.seek(chunk_end)?;
    Ok(())
}

fn deserialize_poly_list<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
    version: u32,
) -> Result<Vec<scene::Polygon>> {
    println!("Reading polygon list");
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
                pub plane: scene::PlanePacked,
            }
            let polygon_data = PolygonData::deserialize(&mut deserializer)?;

            let flags: scene::PolyFlags = match version {
                GOTHIC2_6 => <scene::PolyGothicTwoFlags>::deserialize(&mut deserializer)?.into(),
                GOTHIC1_08K => todo!(),
                _ => return Err(Error::UnknownGameVersion),
            };

            let num_indices = u8::deserialize(&mut deserializer)?;

            let indices = (0..num_indices)
                .map(|_| {
                    let index: scene::Index = match version {
                        GOTHIC2_6 => {
                            <scene::IndexPacked<u32>>::deserialize(&mut deserializer)?.into()
                        }
                        GOTHIC1_08K => {
                            <scene::IndexPacked<u16>>::deserialize(&mut deserializer)?.into()
                        }
                        _ => return Err(Error::UnknownGameVersion),
                    };
                    return Ok(index);
                })
                .collect::<Result<Vec<scene::Index>>>()?;

            Ok(scene::Polygon::new(
                polygon_data.material_index,
                polygon_data.light_map_index,
                polygon_data.plane.into(),
                flags,
                num_indices,
                indices,
            ))
        })
        .collect::<Result<Vec<scene::Polygon>>>()?;
    deserializer.seek(chunk_end)?;
    Ok(polygons)
}

fn read_chunk<R: BinaryRead + AsciiRead>(
    mut reader: R,
    mut builder: Box<SceneMeshBuilder>,
) -> Result<SceneMesh> {
    let mut deserializer = BinaryDeserializer::from(&mut reader);
    let chunk = <mesh::Chunk>::deserialize(&mut deserializer)?;
    let chunk_end = SeekFrom::Current(chunk.length as i64);

    match chunk.id {
        MESH => {
            builder.version = Some(deserialize_mesh::<R>(&mut reader, chunk_end)?);
            read_chunk(reader, builder)
        }
        BBOX3D => {
            let bbox3d = deserialize_bbox3d::<R>(&mut reader, chunk_end)?;
            todo!()
        }
        MAT_LIST => {
            let mat_list = deserialize_mat_list::<R>(&mut reader, chunk_end)?;
            todo!()
        }
        LIGHT_MAP_LIST => {
            let light_mat_list = deserialize_light_mat_list::<R>(&mut reader, chunk_end)?;
            todo!()
        }
        VERT_LIST => {
            let vertices = deserialize_vert_list::<R>(&mut reader, chunk_end)?;
            todo!()
        }
        FEAT_LIST => {
            let features = deserialize_feat_list::<R>(&mut reader, chunk_end)?;
            todo!()
        }
        POLY_LIST => {
            let version = match builder.version {
                Some(v) => v,
                None => {
                    return Err(Error::ExpectedValue(
                        "Expected present version value.".to_owned(),
                    ))
                }
            };
            let polygons = deserialize_poly_list::<R>(&mut reader, chunk_end, version)?;
            todo!()
        }
        MESH_END => Ok(builder.into()),
        _ => {
            deserializer.seek(chunk_end)?;
            todo!()
        }
    }
}

impl SceneMesh {
    pub fn new<R: BinaryRead + AsciiRead>(reader: R, name: &str) -> Result<SceneMesh> {
        read_chunk(reader, Box::new(SceneMeshBuilder::default()))
    }
}
