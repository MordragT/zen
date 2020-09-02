use serde::Deserialize;
use vek::Vec3;

pub mod object;
pub mod scene;
pub struct ChunkHeader {
    start_position: u32,
    size: u32,
    verison: u16,
    object_id: u32,
    name: String,
    class_name: String,
    create_object: bool,
}

#[derive(Deserialize)]
#[repr(C, packed(4))]
pub struct Chunk {
    pub id: u16,
    pub length: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Plane {
    distance: f32,
    normal: Vec3<f32>,
}

// #[derive(Debug)]
// pub struct Vertex {
//     position: (f32, f32, f32),
//     normal: (f32, f32, f32),
//     tex_coord: (f32, f32),
//     color: u32,
// }

// #[derive(Debug)]
// pub struct Triangle {
//     flags: PolyFlags,
//     light_map_index: i16,
//     vertices: [Vertex; 3],
//     submesh_index: i16,
// }
