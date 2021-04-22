use serde::Deserialize;
use vek::Vec3;

#[derive(Deserialize)]
#[repr(C, packed(4))]
pub struct Chunk {
    pub id: u16,
    pub length: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Plane {
    pub distance: f32,
    pub normal: Vec3<f32>,
}
