use std::fs::File;
use zen_core::archive::Vdfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdfs_file = File::open("/home/tom/Steam/common/Gothic II/Data/Anims.vdf")?;
    let vdfs = Vdfs::new(vdfs_file)?;
    println!("{vdfs}");
    Ok(())
}
