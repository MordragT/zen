use std::fs::File;
use std::io::Cursor;
use zen_archive::Vdfs;
use zen_mesh::{gltf, msh::MshMesh, GeneralMesh};
use zen_parser::prelude::*;
use zen_types::path::INSTANCE;

fn main() {
    let vdf_file = File::open(INSTANCE.meshes()).unwrap();
    let vdf = Vdfs::new(vdf_file).unwrap();
    vdf.filter_list("MSH");
    let mesh_entry = vdf.get_by_name("SKYDOME_COLORLAYER.MSH").unwrap();
    println!(
        "Data length: {}, name: {}",
        mesh_entry.data.len(),
        mesh_entry.name
    );
    let cursor = Cursor::new(mesh_entry.data);
    let mesh = MshMesh::new(cursor, &mesh_entry.name).unwrap();
    let mesh = GeneralMesh::from(mesh);
    let gltf = gltf::to_gltf(mesh, gltf::Output::Binary);
}
