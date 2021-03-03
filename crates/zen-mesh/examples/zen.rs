use std::fs::File;
use std::io::Cursor;
use zen_archive::Vdfs;
use zen_mesh::{gltf, msh::SceneMesh, zen::ZenMesh, GeneralMesh};
use zen_parser::prelude::*;
use zen_types::path::INSTANCE;

fn main() {
    let vdf_file = File::open(INSTANCE.worlds()).unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    vdf.list();
    let mesh_entry = vdf.get_by_name("NEWWORLD.ZEN").unwrap();
    println!("Data length: {}", mesh_entry.data.len());
    let cursor = Cursor::new(mesh_entry.data);
    let mesh = ZenMesh::new(cursor, "Newworld").unwrap();
    let mesh = GeneralMesh::from(mesh);
    let gltf = gltf::to_gltf(mesh, gltf::Output::Binary);
}
