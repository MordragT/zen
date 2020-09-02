use crate::error::*;
use serde::Deserialize;
use std::io::{Seek, SeekFrom};
use vek::{Vec2, Vec3, Vec4};
use zen_parser::prelude::*;
use zen_types::{
    material,
    mesh::{self, object},
};

const PROG_MESH: u16 = 45312;
const PROG_MESH_END: u16 = 45567;

const GOTHIC2_6: u32 = 265;
const GOTHIC1_08K: u32 = 9;
/// Holds data of an .mrm file
/// Mrm == Mutli Resolution Mesh
pub struct ObjectMesh {
    pub name: String,
    pub vertices: Vec<Vec3<f32>>,
    pub normals: Vec<Vec3<f32>>,
    pub sub_meshes: Vec<object::SubMesh>,
    pub alpha_test: bool,
    pub bounding_box: (Vec3<f32>, Vec3<f32>),
}

impl ObjectMesh {
    pub fn new<R: BinaryRead + AsciiRead>(reader: R, name: &str) -> Result<ObjectMesh> {
        let mut deserializer = BinaryDeserializer::from(reader);

        // min, max
        let bounding_box;
        let vertices;
        let normals;
        let mut sub_meshes = vec![];
        let alpha_test;

        let chunk = <mesh::Chunk>::deserialize(&mut deserializer)?;
        let chunk_end = SeekFrom::Current(chunk.length as i64);
        match chunk.id {
            PROG_MESH => {
                let _version = u16::deserialize(&mut deserializer)?;
                let data_size = u32::deserialize(&mut deserializer)?;
                let data_seek = deserializer.seek(SeekFrom::Current(0))?;
                deserializer.seek(SeekFrom::Current(data_size as i64))?;

                let num_sub_meshes = u8::deserialize(&mut deserializer)?;
                let main_offsets = <object::Offset>::deserialize(&mut deserializer)?;

                deserializer.len_queue.push(num_sub_meshes as usize);
                let sub_mesh_offsets =
                    <Vec<object::SubMeshOffsets>>::deserialize(&mut deserializer)?;

                let mut ascii_de = AsciiDeserializer::from(deserializer);
                ascii_de.read_header()?;
                deserializer = ascii_de.into();

                let mut materials = (0..num_sub_meshes)
                    .map(|_| {
                        let material: material::Material = {
                            let _name = String::deserialize(&mut deserializer)?;
                            // Skip name and chunk headers
                            let material_header =
                                material::ChunkHeader::deserialize(&mut deserializer)?;

                            // Skip chunk header
                            let _name = String::deserialize(&mut deserializer)?;
                            let _class_name = String::deserialize(&mut deserializer)?;

                            // Save into Vec
                            match material_header.version {
                                material::GOTHIC2 => {
                                    material::AdvancedMaterial::deserialize(&mut deserializer)?
                                        .into()
                                }
                                _ => {
                                    material::BasicMaterial::deserialize(&mut deserializer)?.into()
                                }
                            }
                        };
                        Ok(material)
                    })
                    .collect::<Result<Vec<material::Material>>>()?;

                // TODO gothic 1 should not read byte
                alpha_test = bool::deserialize(&mut deserializer)?;

                // bounding box
                let (min, max) = <(Vec4<f32>, Vec4<f32>)>::deserialize(&mut deserializer)?;
                bounding_box = (min.xyz(), max.xyz());

                deserializer.seek(SeekFrom::Start(
                    data_seek + main_offsets.position.offset as u64,
                ))?;
                deserializer
                    .len_queue
                    .push(main_offsets.position.size as usize);
                vertices = <Vec<Vec3<f32>>>::deserialize(&mut deserializer)?;

                deserializer.seek(SeekFrom::Start(
                    data_seek + main_offsets.normal.offset as u64,
                ))?;
                deserializer
                    .len_queue
                    .push(main_offsets.normal.size as usize);
                normals = <Vec<Vec3<f32>>>::deserialize(&mut deserializer)?;

                for offset in sub_mesh_offsets {
                    deserializer
                        .seek(SeekFrom::Start(data_seek + offset.triangles.offset as u64))?;
                    deserializer.len_queue.push(offset.triangles.size as usize);
                    let triangles = <Vec<Vec3<u16>>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(data_seek + offset.wedges.offset as u64))?;
                    deserializer.len_queue.push(offset.wedges.size as usize);
                    let wedges = <Vec<object::Wedge>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(data_seek + offset.colors.offset as u64))?;
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
                    let triangle_planes = <Vec<mesh::Plane>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(
                        data_seek + offset.triangle_edges.offset as u64,
                    ))?;
                    deserializer
                        .len_queue
                        .push(offset.triangle_edges.size as usize);
                    let triangle_edges = <Vec<Vec3<u16>>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(data_seek + offset.edges.offset as u64))?;
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
                        .seek(SeekFrom::Start(data_seek + offset.wedge_map.offset as u64))?;
                    deserializer.len_queue.push(offset.wedge_map.size as usize);
                    let wedge_map = <Vec<u16>>::deserialize(&mut deserializer)?;

                    let sub_mesh = object::SubMesh::new(
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
                    );

                    sub_meshes.push(sub_mesh);
                }
                deserializer.seek(chunk_end)?;
            }
            _ => {
                return Err(Error::ExpectedIdentifier(format!(
                    "PROG_MESH: {}",
                    PROG_MESH
                )))
            }
        }
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
