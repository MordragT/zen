//! This crate can deserialize [.mrm](Mrm) meshes,
//! and convert them into [Model] objects.
//!
//! ```rust
//! use std::{convert::TryFrom, fs::File, io::Cursor};
//! use zen_archive::Vdfs;
//! use zen_model::{gltf, Model};
//! use zen_mrm::Mrm;
//! use zen_types::path::INSTANCE;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let vdf_file = File::open(INSTANCE.meshes())?;
//! let vdf = Vdfs::new(vdf_file)?;
//! let mesh_entry = vdf
//!     .get_by_name("ORC_MASTERTHRONE.MRM")
//!     .expect("Should be there!");
//! let cursor = Cursor::new(mesh_entry.data);
//! let mesh = Mrm::new(cursor, "ORC_MASTERTHRONE")?;
//! let model = Model::try_from(mesh)?;
//! let _gltf = gltf::to_gltf(mesh, gltf::Output::Binary);
//! #    Ok(())
//! # }
//! ```

pub use error::Error;
use error::Result;
use serde::Deserialize;
use std::io::{Cursor, Seek, SeekFrom};
use sub_mesh::*;
use zen_app::{Asset, AssetLoader};
use zen_material::*;
use zen_math::{Vec2, Vec3, Vec4};
use zen_model::*;
use zen_parser::prelude::*;

mod error;
mod sub_mesh;

const PROG_MESH: u16 = 45312;
//const PROG_MESH_END: u16 = 45567;

pub struct MrmLoader;

impl AssetLoader for MrmLoader {
    type Error = Error;
    fn load(data: &[u8], name: &str) -> Result<Asset> {
        let cursor = Cursor::new(data);
        let mrm = Mrm::new(cursor, name)?;
        let model = Model::try_from(mrm)?;
        Ok(Asset::Model(model))
    }
    fn extensions() -> &'static [&'static str] {
        &["mrm"]
    }
}

/// Holds data of an .mrm file
/// Mrm == Mutli Resolution Mesh
pub struct Mrm {
    pub name: String,
    pub vertices: Vec<Vec3<f32>>,
    pub normals: Vec<Vec3<f32>>,
    pub sub_meshes: Vec<SubMesh>,
    pub alpha_test: bool,
    pub bounding_box: (Vec3<f32>, Vec3<f32>),
}

impl TryFrom<Mrm> for Model {
    type Error = Error;
    fn try_from(object_mesh: Mrm) -> Result<Self> {
        let (object_sub_meshes, object_vertices) = (object_mesh.sub_meshes, object_mesh.vertices);

        let mut materials = Vec::new();
        let meshes = object_sub_meshes
            .into_iter()
            .enumerate()
            .map(|(n, sub_mesh)| {
                let indices = sub_mesh
                    .triangles
                    .into_iter()
                    .map(|v| v.to_array())
                    .flatten()
                    .map(|pos| pos as u32)
                    .collect::<Vec<u32>>();

                let mut mesh = sub_mesh.wedges.into_iter().fold(
                    Mesh {
                        vertices: Vec::new(),
                        num_elements: indices.len() as u32,
                        indices,
                        material: n,
                    },
                    |mut mesh, wedge| {
                        let vertex = Vertex {
                            position: object_vertices[wedge.vertex_index as usize].to_array(),
                            tex_coords: wedge.tex_coord.to_array(),
                            normal: wedge.normal.to_array(),
                        };
                        mesh.vertices.push(vertex);
                        mesh
                    },
                );

                mesh.scale(0.02);

                //let mesh = mesh.pack();
                let material = Material::try_from(&sub_mesh.material)?;
                materials.push(material);

                Ok(mesh)
            })
            .collect::<Result<Vec<Mesh>>>()?;
        Ok(Self {
            name: object_mesh.name,
            meshes,
            materials,
        })
    }
}

impl Mrm {
    /// Creates a new mutli resolution mesh from a reader
    pub fn new<R: BinaryRead + AsciiRead>(reader: R, name: &str) -> Result<Mrm> {
        let mut deserializer = BinaryDeserializer::from(reader);

        #[derive(Deserialize)]
        #[repr(C, packed(4))]
        struct Chunk {
            id: u16,
            length: u32,
        }

        let chunk = <Chunk>::deserialize(&mut deserializer)?;
        let chunk_end = SeekFrom::Current(chunk.length as i64);

        if chunk.id != PROG_MESH {
            return Err(Error::ExpectedIdentifier(format!(
                "PROG_MESH: {}",
                PROG_MESH
            )));
        }

        let _version = u16::deserialize(&mut deserializer)?;
        let data_size = u32::deserialize(&mut deserializer)?;
        let data_seek = deserializer.seek(SeekFrom::Current(0))?;
        deserializer
            .parser
            .seek(SeekFrom::Current(data_size as i64))?;

        let num_sub_meshes = u8::deserialize(&mut deserializer)?;
        let main_offsets = <Offset>::deserialize(&mut deserializer)?;

        deserializer.len_queue.push(num_sub_meshes as usize);
        let sub_mesh_offsets = <Vec<SubMeshOffsets>>::deserialize(&mut deserializer)?;

        // let mut ascii_de = AsciiDeserializer::from(deserializer);
        // ascii_de.read_header()?;
        // deserializer = ascii_de.into();
        let _header = Reader::from(&mut deserializer.parser).read_header()?;

        let mut materials = (0..num_sub_meshes)
            .map(|_| {
                let material: GeneralMaterial = {
                    let _name = String::deserialize(&mut deserializer)?;
                    // Skip name and chunk headers
                    let material_header = ChunkHeader::deserialize(&mut deserializer)?;

                    // Skip chunk header
                    let _name = String::deserialize(&mut deserializer)?;
                    let _class_name = String::deserialize(&mut deserializer)?;

                    // Save into Vec
                    match material_header.version {
                        zen_material::GOTHIC2 => {
                            AdvancedMaterial::deserialize(&mut deserializer)?.into()
                        }
                        _ => BasicMaterial::deserialize(&mut deserializer)?.into(),
                    }
                };
                Ok(material)
            })
            .collect::<Result<Vec<GeneralMaterial>>>()?;

        // TODO gothic 1 should not read byte
        let alpha_test = bool::deserialize(&mut deserializer)?;

        // bounding box
        let (min, max) = <(Vec4<f32>, Vec4<f32>)>::deserialize(&mut deserializer)?;
        let bounding_box = (min.xyz(), max.xyz());

        deserializer.seek(SeekFrom::Start(
            data_seek + main_offsets.position.offset as u64,
        ))?;
        deserializer
            .len_queue
            .push(main_offsets.position.size as usize);
        let vertices = <Vec<Vec3<f32>>>::deserialize(&mut deserializer)?;

        deserializer.seek(SeekFrom::Start(
            data_seek + main_offsets.normal.offset as u64,
        ))?;
        deserializer
            .len_queue
            .push(main_offsets.normal.size as usize);
        let normals = <Vec<Vec3<f32>>>::deserialize(&mut deserializer)?;

        let sub_meshes = sub_mesh_offsets
            .into_iter()
            .map(|offset| {
                deserializer
                    .parser
                    .seek(SeekFrom::Start(data_seek + offset.triangles.offset as u64))?;
                deserializer.len_queue.push(offset.triangles.size as usize);
                let triangles = <Vec<Vec3<u16>>>::deserialize(&mut deserializer)?;

                deserializer
                    .parser
                    .seek(SeekFrom::Start(data_seek + offset.wedges.offset as u64))?;
                deserializer.len_queue.push(offset.wedges.size as usize);
                let wedges = <Vec<Wedge>>::deserialize(&mut deserializer)?;

                deserializer
                    .parser
                    .seek(SeekFrom::Start(data_seek + offset.colors.offset as u64))?;
                deserializer.len_queue.push(offset.colors.size as usize);
                let colors = <Vec<f32>>::deserialize(&mut deserializer)?;

                deserializer.seek(SeekFrom::Start(
                    data_seek + offset.triangle_plane_indices.offset as u64,
                ))?;
                deserializer
                    .len_queue
                    .push(offset.triangle_plane_indices.size as usize);
                let triangle_plane_indices = <Vec<u16>>::deserialize(&mut deserializer)?;

                deserializer.seek(SeekFrom::Start(
                    data_seek + offset.triangle_planes.offset as u64,
                ))?;
                deserializer
                    .len_queue
                    .push(offset.triangle_planes.size as usize);
                let triangle_planes = <Vec<Plane>>::deserialize(&mut deserializer)?;

                deserializer.seek(SeekFrom::Start(
                    data_seek + offset.triangle_edges.offset as u64,
                ))?;
                deserializer
                    .len_queue
                    .push(offset.triangle_edges.size as usize);
                let triangle_edges = <Vec<Vec3<u16>>>::deserialize(&mut deserializer)?;

                deserializer
                    .parser
                    .seek(SeekFrom::Start(data_seek + offset.edges.offset as u64))?;
                deserializer.len_queue.push(offset.edges.size as usize);
                let edges = <Vec<Vec2<u16>>>::deserialize(&mut deserializer)?;

                deserializer.seek(SeekFrom::Start(
                    data_seek + offset.edge_scores.offset as u64,
                ))?;
                deserializer
                    .len_queue
                    .push(offset.edge_scores.size as usize);
                let edge_scores = <Vec<f32>>::deserialize(&mut deserializer)?;

                deserializer
                    .parser
                    .seek(SeekFrom::Start(data_seek + offset.wedge_map.offset as u64))?;
                deserializer.len_queue.push(offset.wedge_map.size as usize);
                let wedge_map = <Vec<u16>>::deserialize(&mut deserializer)?;

                Ok(SubMesh::new(
                    materials.remove(0),
                    triangles,
                    wedges,
                    colors,
                    triangle_plane_indices,
                    triangle_planes,
                    triangle_edges,
                    wedge_map,
                    edges,
                    edge_scores,
                ))
            })
            .collect::<Result<Vec<SubMesh>>>()?;
        deserializer.seek(chunk_end)?;
        Ok(Self {
            name: name.to_string(),
            vertices,
            normals,
            sub_meshes,
            alpha_test,
            bounding_box,
        })
    }
}
