use serde::Deserialize;
use std::fs::File;
use zen_parser::prelude::{read_header, AsciiDecoder};

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
    let mut file =
        File::open("/home/tom/Git/Rust/zen-loader/crates/zen-parser/examples/example.zen")?;
    let header = read_header(&mut file)?;
    let mut de = AsciiDecoder::from(file);
    let lens_flare = LensFlareFX::deserialize(&mut de)?;
    println!("{:?}", lens_flare);
    Ok(())
}
