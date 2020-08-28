use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::Read;
use zen_loader::deserializer::ascii::AsciiDeserializer;

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

fn main() {
    let mut file = File::open("/home/tom/Git/zen-loader/examples/example.zen").unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).unwrap();
    let mut de = AsciiDeserializer::new(&mut contents).unwrap();
    let lens_flare = LensFlareFX::deserialize(&mut de).unwrap();
    println!("{:?}", lens_flare);
}
