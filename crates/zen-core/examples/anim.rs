use std::fs::File;
use zen_archive::Vdfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Anims.vdf")?;
    let vdf = Vdfs::new(vdf_file)?;
    vdf.list();
    Ok(())
}
