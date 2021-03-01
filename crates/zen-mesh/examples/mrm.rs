use std::fs::File;
use std::io::Cursor;
use zen_archive::Vdfs;
use zen_mesh::{gltf, mrm::MrmMesh, GeneralMesh};
use zen_parser::prelude::*;
use zen_types::path::INSTANCE;

fn main() {
    let vdf_file = File::open(INSTANCE.meshes()).unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    //vdf.list();
    let mesh_entry = vdf.get_by_name("ORC_MASTERTHRONE.MRM").unwrap();
    println!("Data length: {}", mesh_entry.data.len());
    let cursor = Cursor::new(mesh_entry.data);
    let object_mesh = MrmMesh::new(cursor, "ORC_MASTERTHRONE").unwrap();
    let mesh = GeneralMesh::from(object_mesh);
    let gltf = gltf::to_gltf(mesh, gltf::Output::Binary);
}
