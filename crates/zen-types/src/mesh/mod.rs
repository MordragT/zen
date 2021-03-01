use serde::Deserialize;
use vek::Vec3;

pub mod object;
pub mod scene;

// pub struct ChunkHeader {
//     pub start_position: u32,
//     pub size: u32,
//     pub verison: u16,
//     pub object_id: u32,
//     pub name: String,
//     pub class_name: String,
//     pub create_object: bool,
// }

// impl ChunkHeader {
//     pub fn read<R: BinaryRead + AsciiRead>(reader: &mut R) -> Self {

//     }
// }

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
