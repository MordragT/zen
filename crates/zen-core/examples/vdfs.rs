use std::fs::File;
use zen_core::archive::Vdfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdfs_file = File::open("/home/tom/Steam/common/Gothic II/Data/Worlds.vdf")?;
    let vdfs = Vdfs::new(vdfs_file)?;
    println!("{vdfs}");
    for entry in vdfs.entries().unwrap() {
        println!("{}", entry.name());
    }
    // let entry = vdf
    //     .get_by_name_slice("MOSTORCTHRONE02")
    //     .expect("Should be there!");
    // println!("{}", entry);
    Ok(())
}
