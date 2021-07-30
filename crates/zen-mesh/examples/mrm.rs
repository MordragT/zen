use std::{convert::TryFrom, fs::File, io::Cursor};
use zen_archive::Vdfs;
use zen_mesh::gltf;
use zen_mesh::{mrm::MrmMesh, Model};
use zen_types::path::INSTANCE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_file = File::open(INSTANCE.meshes())?;
    let vdf = Vdfs::new(vdf_file)?;
    let mesh_entry = vdf
        .get_by_name("ORC_MASTERTHRONE.MRM")
        .expect("Should be there!");
    let cursor = Cursor::new(mesh_entry.data);
    let mesh = MrmMesh::new(cursor, "ORC_MASTERTHRONE")?;
    let model = Model::try_from(mesh)?;
    for mesh in model.clone().meshes {
        println!("Material: {}", mesh.material);
        println!("Number Elements: {}", mesh.num_elements);
        println!("Indices Len: {}", mesh.indices.len());
        println!("Vertices Len: {}", mesh.vertices.len());
    }
    let _gltf = gltf::to_gltf(model, gltf::Output::Binary);
    Ok(())
}
