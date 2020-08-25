use crate::FromReader;
use serde::Deserialize;
use std::io::Read;

#[derive(FromReader, Deserialize, Debug)]
pub struct Float2(pub f32, pub f32);
#[derive(FromReader, Deserialize, Debug)]
pub struct Float3(pub f32, pub f32, pub f32);
#[derive(FromReader, Deserialize, Debug)]
pub struct Float4(pub f32, pub f32, pub f32, pub f32);
