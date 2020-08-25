use std::fs::File;
use zen_loader::vdfs::Vdfs;

fn main() {
    let vdf_file = File::open("/home/tom/.steam/steam/steamapps/common/Gothic/meshes.VDF").unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    vdf.list();
}
