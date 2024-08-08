use image::codecs::png::PngEncoder;
use std::{fs::File, io::BufReader};
use zen_vdfs::VdfsArchive;
use zen_ztex::ZTex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(format!("{}/Data/Textures.vdf", zen_core::GOTHIC2_PATH))?;
    let reader = BufReader::new(file);
    let mut vdfs = VdfsArchive::from_reader(reader)?;

    let entry = vdfs.get("IT_POTIONS_01-C.TEX").expect("should be present");
    let data = vdfs.fetch_mut(&entry)?;

    let mut ztex = ZTex::from_slice(&data)?;
    println!("{ztex}");

    let encoder = PngEncoder::new(File::create("it_potions_01.png")?);
    ztex.encode(encoder)?;

    Ok(())
}
