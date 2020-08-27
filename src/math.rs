use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Float2(pub f32, pub f32);
#[derive(Deserialize, Debug)]
pub struct Float3(pub f32, pub f32, pub f32);
#[derive(Deserialize, Debug)]
pub struct Float4(pub f32, pub f32, pub f32, pub f32);
