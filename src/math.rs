use crate::FromReader;
use serde::Deserialize;
use std::io::Read;

#[derive(FromReader, Deserialize, Debug)]
pub struct Float2(f32, f32);
#[derive(FromReader, Deserialize, Debug)]
pub struct Float3(f32, f32, f32);
#[derive(FromReader, Deserialize, Debug)]
pub struct Float4(f32, f32, f32, f32);
