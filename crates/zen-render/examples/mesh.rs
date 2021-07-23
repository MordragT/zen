use std::{convert::TryFrom, fs::File, io::Cursor};
use zen_archive::Vdfs;
use zen_mesh::{gltf, Mesh};
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
    let _gltf = gltf::to_gltf(model.clone(), gltf::Output::Binary);
    let mesh = model.meshes[3].clone();
    println!("{:?}", &mesh.positions[0..3]);
    zen_render::run(mesh);

    // let mesh = Mesh {
    //     positions: vec![
    //         -0.0868241,
    //         0.49240386,
    //         0.0,
    //         0.49513406,
    //         0.06958647,
    //         0.0,
    //         -0.21918549,
    //         -0.44939706,
    //         0.0,
    //         0.35966998,
    //         -0.3473291,
    //         0.0,
    //         0.44147372,
    //         0.2347359,
    //         0.0,
    //     ],
    //     indices: vec![0, 1, 4, 1, 2, 4, 2, 3, 4, 0],
    //     material: 0,
    //     normals: vec![],
    //     tex_coords: vec![],
    //     num_elements: 10,
    // };
    // zen_render::run(mesh);
    Ok(())
}
