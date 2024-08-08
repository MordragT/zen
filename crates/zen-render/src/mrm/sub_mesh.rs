use crate::{
    material::ZenMaterial,
    math::{Vec2, Vec3},
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Plane {
    pub distance: f32,
    pub normal: Vec3<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Wedge {
    pub normal: Vec3<f32>,
    pub tex_coord: Vec2<f32>,
    pub vertex_index: u16,
    pub alignment: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct SubMesh {
    pub material: ZenMaterial,
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
