use std::fs::File;
use zen_core::archive::Vdfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdfs_file = File::open("/opt/SteamLibrary/steamapps/common/Gothic II/Data/Textures.vdf")?;
    let vdfs = Vdfs::new(vdfs_file)?;
    println!("{vdfs}");
    // let entry = vdf
    //     .get_by_name_slice("MOSTORCTHRONE02")
    //     .expect("Should be there!");
    // println!("{}", entry);
    Ok(())
}
