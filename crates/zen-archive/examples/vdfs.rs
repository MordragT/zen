use std::fs::File;
use zen_archive::Vdfs;

fn main() {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic/Data/worlds.VDF").unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    vdf.list();
}
