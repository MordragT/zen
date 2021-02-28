use std::fs::File;
use zen_archive::Vdfs;

fn main() {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Textures.vdf").unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    let entry = vdf.get_by_name_slice("MOSTORCTHRONE02").unwrap();
    dbg!(entry.name);
    //vdf.list();
}
