use std::fs::File;
use zen_archive::Vdfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Textures.vdf")?;
    let vdf = Vdfs::new(vdf_file)?;
    let entry = vdf
        .get_by_name_slice("MOSTORCTHRONE02")
        .expect("Should be there!");
    println!("{}", entry);
    Ok(())
}
