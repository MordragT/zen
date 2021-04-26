use std::{convert::TryFrom, fs::File, io::Cursor};
use zen_archive::Vdfs;
use zen_mesh::{gltf, msh::MshMesh, GeneralMesh};
use zen_types::path::INSTANCE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_file = File::open(INSTANCE.meshes())?;
    let vdf = Vdfs::new(vdf_file)?;
    vdf.filter_list("MSH");
    let mesh_entry = vdf.get_by_name("MFX_FEAR4.MSH").expect("Should be there!");
    let cursor = Cursor::new(mesh_entry.data);
    let mesh = MshMesh::new(cursor, &mesh_entry.name)?;
    let mesh = GeneralMesh::try_from(mesh)?;
    let _gltf = gltf::to_gltf(mesh, gltf::Output::Binary);
    Ok(())
}
