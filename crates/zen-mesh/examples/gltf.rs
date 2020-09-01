use std::fs::File;
use std::io::Cursor;
use zen_archive::Vdfs;
use zen_mesh::{Mesh, Output};
use zen_parser::prelude::*;

fn main() {
    let vdf_file = File::open("/home/tom/Steam/common/Gothic II/Data/Meshes.vdf").unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    let mesh_entry = vdf.get_by_name("ORC_BROKENCART.MRM").unwrap().unwrap();
    println!("Data length: {}", mesh_entry.data.len());
    let cursor = Cursor::new(mesh_entry.data);
    let mesh = Mesh::from_mrm(cursor).unwrap();
    let gltf = mesh.to_gltf(Output::Standard);
    println!("{:?}", gltf);
}
