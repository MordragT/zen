use std::{fs::File, io::Write, path::Path};
use zen_archive::Vdfs;

fn main() {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Sounds.vdf").unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    let entry = vdf.get_by_name_slice("CHAPTER_01.WAV").unwrap();
    let mut audio_file =
        File::create("/home/tom/Git/zen-loader/files/audio/chapter_01.wav").unwrap();
    audio_file.write(&entry.data).unwrap();
    // dbg!(entry.name);
    //vdf.list();
}
