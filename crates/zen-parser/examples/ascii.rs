use serde::Deserialize;
use std::fs::File;
use zen_parser::prelude::AsciiDeserializer;

#[derive(Deserialize, Debug)]
struct LensFlareFX {
    name: String,
    num_flares: i32,
    textures: Vec<Texture>,
}
#[derive(Deserialize, Debug)]
struct Texture {
    name: String,
    kind: i32,
    size: f32,
    alpha: f32,
    range_min: f32,
    //range_max: f32,
    pos_scale: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("/home/tom/Git/Rust/zen-loader/crates/zen-parser/examples/example.zen")?;
    let mut de = AsciiDeserializer::from(file);
    de.read_header()?;
    let lens_flare = LensFlareFX::deserialize(&mut de)?;
    println!("{:?}", lens_flare);
    Ok(())
}
