use std::{fs::File, io::Write};
use zen_vdfs::VdfsArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdfs_file = File::open(format!("{}/Data/Sounds.vdf", zen_core::GOTHIC2_PATH))?;
    let vdfs = VdfsArchive::new(vdfs_file)?;

    let entry = vdfs.get("CHAPTER_01.WAV").expect("Should be there!");
    let buf = vdfs.fetch(&entry)?;

    let mut audio_file = File::create("chapter_01.wav")?;
    audio_file.write(&buf)?;
    Ok(())
}
