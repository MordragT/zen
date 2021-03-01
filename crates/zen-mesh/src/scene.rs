use crate::error::*;
use material::GeneralMaterial;
use serde::Deserialize;
use std::io::{Read, Seek, SeekFrom};
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
}

impl From<Box<SceneMeshBuilder>> for SceneMesh {
    fn from(builder: Box<SceneMeshBuilder>) -> Self {
        todo!()
    }
}

#[derive(Default)]
pub struct SceneMeshBuilder {
    pub name: String,
    pub version: Option<u32>,
    //pub mesh: Option<()>,
    //pub bbox3d: Option<()>,
    pub materials: Option<Vec<GeneralMaterial>>,
    pub light_maps: Option<()>,
    pub vertices: Option<Vec<Vec3<f32>>>,
    pub features: Option<Vec<scene::FeatureChunk>>,
    pub polygons: Option<Vec<scene::Polygon>>,
}

fn deserialize_version<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<u32> {
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

fn deserialize_materials<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<Vec<material::GeneralMaterial>> {
    println!("Reading material list");
    let mut deserializer = BinaryDeserializer::from(reader);
    let _header = Reader::from(&mut deserializer.parser).read_header()?;

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

fn deserialize_light_maps<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<()> {
    println!("Reading light maps");
    let mut deserializer = BinaryDeserializer::from(reader);
    deserializer.seek(chunk_end)?;
    Ok(())
}

fn deserialize_vertices<R: BinaryRead + AsciiRead>(
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

fn deserialize_features<R: BinaryRead + AsciiRead>(
    reader: &mut R,
    chunk_end: SeekFrom,
) -> Result<Vec<scene::FeatureChunk>> {
    let mut deserializer = BinaryDeserializer::from(reader);
    println!("Reading feature list");
    let num_feats = u32::deserialize(&mut deserializer)?;
    deserializer.len_queue.push(num_feats as usize);
    let features = <Vec<scene::FeatureChunk>>::deserialize(&mut deserializer)?;
    deserializer.seek(chunk_end)?;
    Ok(features)
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
    println!("Read chunk...");

    let mut deserializer = BinaryDeserializer::from(&mut reader);
    let chunk = <mesh::Chunk>::deserialize(&mut deserializer)?;
    let chunk_end = SeekFrom::Current(chunk.length as i64);

    dbg!(chunk.id);
    dbg!(chunk_end);

    match chunk.id {
        MESH => {
            builder.version = Some(deserialize_version::<R>(&mut reader, chunk_end)?);
            read_chunk(reader, builder)
        }
        BBOX3D => {
            let bbox3d = deserialize_bbox3d::<R>(&mut reader, chunk_end)?;
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
                    return Err(Error::ExpectedValue(
                        "Expected present version value.".to_owned(),
                    ))
                }
            };
            builder.polygons = Some(deserialize_poly_list::<R>(&mut reader, chunk_end, version)?);
            read_chunk(reader, builder)
        }
        MESH_END => Ok(builder.into()),
        _ => {
            println!("Unknown chunk.");
            deserializer.seek(chunk_end)?;
            read_chunk(reader, builder)
        }
    }
}

impl SceneMesh {
    pub fn new<R: BinaryRead + AsciiRead>(mut reader: R, name: &str) -> Result<SceneMesh> {
        let _header = Reader::from(&mut reader).read_header()?;
        //let mut deserializer = BinaryDeserializer::from(&mut reader);
        //let _binsafe_header = <binsafe::BinSafeHeader>::deserialize(&mut deserializer)?;

        // // skip bytes dunno what it is
        // let mut buf = [0_u8; 182];
        // reader.read(&mut buf);
        // //buf.into_iter().for_each(|u| print!("{}", *u));

        // let header = AsciiDeserializer::from(&mut reader).read_header()?;
        // dbg!(header);

        // let mut buf = [0_u8; 182];
        // reader.read(&mut buf);
        // buf.into_iter().for_each(|u| print!("{}", *u as char));

        read_chunk(
            reader,
            Box::new(SceneMeshBuilder {
                name: name.to_owned(),
                ..Default::default()
            }),
        )
    }
}
