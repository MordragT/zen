use std::{fs::File, io::BufReader};

use zen_vdfs::VdfsArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(format!("{}/Data/Worlds.vdf", zen_core::GOTHIC2_PATH))?;
    let reader = BufReader::new(file);
    let vdfs = VdfsArchive::from_reader(reader)?;

    println!("{vdfs}");

    for entry in vdfs.entries() {
        println!("{}", entry.name);
    }
    Ok(())
}
