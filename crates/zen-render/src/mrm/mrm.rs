use zen_parser::{binary::BinaryResult, prelude::*};

use super::{header::*, sub_mesh::*, MrmResult};
use crate::{
    material::ZenMaterial,
    math::{Vec2, Vec3, Vec4},
};

/// Holds data of an .mrm file
/// Mrm == Mutli Resolution Mesh
#[derive(Debug)]
pub struct Mrm {
    pub vertices: Vec<Vec3<f32>>,
    pub normals: Vec<Vec3<f32>>,
    pub sub_meshes: Vec<SubMesh>,
    pub alpha_test: bool,
    pub bounding_box: (Vec3<f32>, Vec3<f32>),
}

impl Mrm {
    /// Creates a new mutli resolution mesh from a reader
    pub fn from_decoder<R>(decoder: &mut BinaryDecoder<R>) -> MrmResult<Mrm>
    where
        R: BinaryRead,
    {
        let header = decoder.decode::<MrmHeader>()?;
        header.validate()?;

        let data_pos = decoder.position()?;
        decoder.offset_position(header.size as i64)?;

        let sub_mesh_count = decoder.decode::<u8>()?;
        let offsets = decoder.decode::<Offsets>()?;

        decoder.push_size(sub_mesh_count as usize);
        let sub_mesh_offsets = decoder.decode::<Vec<SubMeshOffsets>>()?;

        let _header = decoder.decode_header()?;

        let mut materials = (0..sub_mesh_count)
            .map(|_| ZenMaterial::from_decoder(decoder))
            .collect::<BinaryResult<Vec<ZenMaterial>>>()?;

        // TODO gothic 1 should not read byte
        let alpha_test = decoder.decode::<bool>()?;

        // bounding box
        let (min, max) = decoder.decode::<(Vec4<f32>, Vec4<f32>)>()?;
        let bounding_box = (min.xyz(), max.xyz());

        decoder.set_position(data_pos + offsets.position.offset as u64)?;
        decoder.push_size(offsets.position.size as usize);
        let vertices = decoder.decode::<Vec<Vec3<f32>>>()?;

        decoder.set_position(data_pos + offsets.normal.offset as u64)?;
        decoder.push_size(offsets.normal.size as usize);
        let normals = decoder.decode::<Vec<Vec3<f32>>>()?;

        let sub_meshes = sub_mesh_offsets
            .into_iter()
            .map(|offset| {
                decoder.set_position(data_pos + offset.triangles.offset as u64)?;
                decoder.push_size(offset.triangles.size as usize);
                let triangles = decoder.decode::<Vec<Vec3<u16>>>()?;

                decoder.set_position(data_pos + offset.wedges.offset as u64)?;
                decoder.push_size(offset.wedges.size as usize);
                let wedges = decoder.decode::<Vec<Wedge>>()?;

                decoder.set_position(data_pos + offset.colors.offset as u64)?;
                decoder.push_size(offset.colors.size as usize);
                let colors = decoder.decode::<Vec<f32>>()?;

                decoder.set_position(data_pos + offset.triangle_plane_indices.offset as u64)?;
                decoder.push_size(offset.triangle_plane_indices.size as usize);
                let triangle_plane_indices = decoder.decode::<Vec<u16>>()?;

                decoder.set_position(data_pos + offset.triangle_planes.offset as u64)?;
                decoder.push_size(offset.triangle_planes.size as usize);
                let triangle_planes = decoder.decode::<Vec<Plane>>()?;

                decoder.set_position(data_pos + offset.triangle_edges.offset as u64)?;
                decoder.push_size(offset.triangle_edges.size as usize);
                let triangle_edges = decoder.decode::<Vec<Vec3<u16>>>()?;

                decoder.set_position(data_pos + offset.edges.offset as u64)?;
                decoder.push_size(offset.edges.size as usize);
                let edges = decoder.decode::<Vec<Vec2<u16>>>()?;

                decoder.set_position(data_pos + offset.edge_scores.offset as u64)?;
                decoder.push_size(offset.edge_scores.size as usize);
                let edge_scores = decoder.decode::<Vec<f32>>()?;

                decoder.set_position(data_pos + offset.wedge_map.offset as u64)?;
                decoder.push_size(offset.wedge_map.size as usize);
                let wedge_map = decoder.decode::<Vec<u16>>()?;

                Ok(SubMesh {
                    material: materials.remove(0),
                    triangles,
                    wedges,
                    colors,
                    triangle_plane_indices,
                    triangle_planes,
                    triangle_edges,
                    wedge_map,
                    edges,
                    edge_scores,
                })
            })
            .collect::<MrmResult<Vec<SubMesh>>>()?;

        decoder.set_position(data_pos + header.chunk_length())?;

        Ok(Self {
            vertices,
            normals,
            sub_meshes,
            alpha_test,
            bounding_box,
        })
    }
}
