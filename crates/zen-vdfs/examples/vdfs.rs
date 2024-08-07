use std::fs::File;

use zen_vdfs::VdfsArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdfs_file = File::open(format!("{}/Data/Worlds.vdf", zen_core::GOTHIC2_PATH))?;
    let vdfs = VdfsArchive::new(vdfs_file)?;

    println!("{vdfs}");

    for entry in vdfs.entries() {
        println!("{}", entry.name);
    }
    Ok(())
}
