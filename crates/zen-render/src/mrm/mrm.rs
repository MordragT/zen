use std::io;

use bevy::asset::LoadContext;
use gltf_json as json;
use zen_parser::prelude::*;

use super::{header::*, mesh::*, MrmResult};
use crate::{
    material::{ZMat, ZMatResult},
    math::{Vec2, Vec3, Vec4},
};

/// Holds data of an .mrm file
/// Mrm == Mutli Resolution Mesh
#[derive(Debug)]
pub struct Mrm {
    pub vertices: Vec<Vec3<f32>>,
    pub normals: Vec<Vec3<f32>>,
    pub materials: Vec<ZMat>,
    pub meshes: Vec<MrmMesh>,
    pub alpha_test: bool,
    pub bounding_box: (Vec3<f32>, Vec3<f32>),
}

impl Mrm {
    pub fn from_reader<R>(reader: R) -> MrmResult<Self>
    where
        R: io::BufRead + io::Seek,
    {
        let mut decoder = BinaryDecoder::from_reader(reader);
        Self::from_decoder(&mut decoder)
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> MrmResult<Self> {
        let mut decoder = BinaryDecoder::from_bytes(bytes);
        Self::from_decoder(&mut decoder)
    }

    /// Creates a new mutli resolution mesh from a reader
    pub fn from_decoder<R>(decoder: &mut BinaryDecoder<R>) -> MrmResult<Self>
    where
        R: BinaryRead,
    {
        let header = decoder.decode::<MrmHeader>()?;
        header.validate()?;

        let data_pos = decoder.position()?;
        decoder.offset_position(header.size as i64)?;

        let mesh_count = decoder.decode::<u8>()?;
        let offsets = decoder.decode::<Offsets>()?;

        decoder.push_size(mesh_count as usize);
        let mesh_offsets = decoder.decode::<Vec<SubMeshOffsets>>()?;

        let _header = decoder.decode_header()?;

        let materials = (0..mesh_count)
            .map(|_| ZMat::from_decoder(decoder))
            .collect::<ZMatResult<Vec<ZMat>>>()?;

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

        let meshes = mesh_offsets
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

                Ok(MrmMesh {
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
            .collect::<MrmResult<Vec<MrmMesh>>>()?;

        decoder.set_position(data_pos + header.chunk_length())?;

        Ok(Self {
            vertices,
            normals,
            materials,
            meshes,
            alpha_test,
            bounding_box,
        })
    }
}

impl Mrm {
    pub fn into_gltf(self, load_context: &mut LoadContext<'_>) -> json::Root {
        todo!()
    }
}
