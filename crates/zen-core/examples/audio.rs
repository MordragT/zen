use std::{
    fs::File,
    io::{Read, Write},
};
use zen_core::archive::Vdfs;

const INPUT: &'static str = "/home/tom/Steam/common/Gothic II/Data/Sounds.vdf";
const OUTPUT: &'static str = "/home/tom/Desktop/zen/assets/orig/chapter_01.wav";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdfs_file = File::open(INPUT)?;
    let vdfs = Vdfs::new(vdfs_file)?;

    let mut entry = vdfs
        .entries()?
        .find(|entry| entry.name() == "CHAPTER_01.WAV")
        .expect("Should be there!");

    let mut buf = Vec::new();
    entry.read_to_end(&mut buf)?;

    let mut audio_file = File::create(OUTPUT)?;
    audio_file.write(&buf)?;
    Ok(())
}
