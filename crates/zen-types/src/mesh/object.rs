use super::Plane;
use crate::material::Material;
use serde::Deserialize;
use vek::{Vec2, Vec3};

#[derive(Deserialize, Debug)]
pub struct DataEntry {
    pub offset: u32,
    pub size: u32,
}
#[derive(Deserialize, Debug)]
pub struct Offset {
    pub position: DataEntry,
    pub normal: DataEntry,
}
#[derive(Deserialize, Debug)]
pub struct SubMeshOffsets {
    pub triangles: DataEntry,
    pub wedges: DataEntry,
    pub colors: DataEntry,
    pub triangle_plane_indices: DataEntry,
    pub triangle_planes: DataEntry,
    pub wedge_map: DataEntry,
    pub vertex_updates: DataEntry,
    pub triangle_edges: DataEntry,
    pub edges: DataEntry,
    pub edge_scores: DataEntry,
}

#[repr(C)]
#[derive(Deserialize, Debug, Clone)]
pub struct Wedge {
    pub normal: Vec3<f32>,
    pub tex_coord: Vec2<f32>,
    pub vertex_index: u16,
    pub alignment: u16,
}

#[derive(Debug, Clone)]
pub struct SubMesh {
    pub material: Material,
    pub triangles: Vec<Vec3<u16>>,
    pub wedges: Vec<Wedge>,
    pub colors: Vec<f32>,
    pub triangle_plane_indices: Vec<u16>,
    pub triangle_planes: Vec<Plane>,
    pub triangle_edges: Vec<Vec3<u16>>,
    pub wedge_map: Vec<u16>,
    pub edges: Vec<Vec2<u16>>,
    pub edge_scores: Vec<f32>,
}

impl SubMesh {
    pub fn new(
        material: Material,
        triangles: Vec<Vec3<u16>>,
        wedges: Vec<Wedge>,
        colors: Vec<f32>,
        triangle_plane_indices: Vec<u16>,
        triangle_planes: Vec<Plane>,
        triangle_edges: Vec<Vec3<u16>>,
        wedge_map: Vec<u16>,
        edges: Vec<Vec2<u16>>,
        edge_scores: Vec<f32>,
    ) -> Self {
        Self {
            material,
            triangles,
            wedges,
            colors,
            triangle_plane_indices,
            triangle_planes,
            triangle_edges,
            wedge_map,
            edges,
            edge_scores,
        }
    }
}
