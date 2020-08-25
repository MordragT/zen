use std::fs::File;
use zen_loader::texture;
use zen_loader::vdfs::Vdfs;

fn main() {
    let vdf_file =
        File::open("/home/tom/.steam/steam/steamapps/common/Gothic/textures.VDF").unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    let yellow_tex = vdf.get_by_name("YELLOW-C.TEX").unwrap();
    let data = yellow_tex.data;
    let dds = texture::convert_ztex_to_dds(data.as_slice()).unwrap();
    dbg!(dds);
}
