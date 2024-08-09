use crate::math::{Vec2, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Plane {
    pub distance: f32,
    pub normal: Vec3<f32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Wedge {
    pub normal: Vec3<f32>,
    pub tex_coord: Vec2<f32>,
    pub vertex_index: u16,
    pub alignment: u16,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct MrmMesh {
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
