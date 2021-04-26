use std::{fs::File, io::Write};
use zen_archive::Vdfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Sounds.vdf")?;
    let vdf = Vdfs::new(vdf_file)?;
    let entry = vdf
        .get_by_name_slice("CHAPTER_01.WAV")
        .expect("Should be there!");
    let mut audio_file = File::create("/home/tom/Git/zen-loader/files/audio/chapter_01.wav")?;
    audio_file.write(&entry.data)?;
    Ok(())
}
