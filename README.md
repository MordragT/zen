## Zen Loader

- Rewrite of the [Zenlib](https://github.com/ataulien/ZenLib) in Rust
- Provides a library to open zengine specific data formats
- And export data to modern data formats

- At the moment you can open [VDFS-Archives](zen_archive), and export [Multiresolution-Meshes](zen_mesh::mrm) (.mrm) aswell as normal [Zengin-Meshes](zen_mesh::msh) (.msh) from the archives to gltf files.
- The corresponding [textures](zen_texture) will also be exported (similiar to dds textures), or you can export those textures one by one aswell.
- I am working on the export of [Zengin World Scenes](zen_mesh::zen) (.zen) to gltf and a [Daedalus](zen_daedalus) (scripting language) virtual machine to execute the bytecode.

#### Links

- [crate](https://crates.io/crates/zen-loader)
- [documentation](https://docs.rs/zen-loader/0.1.0/zen-loader)

## Examples

#### Vdfs Archive

```rust
use std::{fs::File, io::Write};
use zen_archive::Vdfs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_file = File::open("/home/user/../Gothic II/Data/Sounds.vdf")?;
    let vdf = Vdfs::new(vdf_file)?;
    let entry = vdf.get_by_name_slice("CHAPTER_01.WAV").unwrap();
    let mut audio_file = File::create("/home/user/../files/audio/chapter_01.wav")?;
    audio_file.write(&entry.data)?;
    Ok(())
}

```

#### Daedalus Bytecode

```rust
use std::fs::File;
use zen_daedalus::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file =
        File::open("/home/user/../Gothic II/_work/Data/Scripts/_compiled/CAMERA.DAT")?;

    let code = Code::new(file)?;
    let mut machine = Machine::new(code);
    machine.run();
}
```

#### Multi Resolution Mesh

```rust
use std::{convert::TryFrom, fs::File, io::Cursor};
use zen_archive::Vdfs;
use zen_mesh::{gltf, mrm::MrmMesh, GeneralMesh};
use zen_types::path::INSTANCE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_file = File::open(INSTANCE.meshes())?;
    let vdf = Vdfs::new(vdf_file)?;
    let mesh_entry = vdf
        .get_by_name("ORC_MASTERTHRONE.MRM")
        .expect("Should be there!");
    let cursor = Cursor::new(mesh_entry.data);
    let mesh = MrmMesh::new(cursor, "ORC_MASTERTHRONE")?;
    let mesh = GeneralMesh::try_from(mesh)?;
    let _gltf = gltf::to_gltf(mesh, gltf::Output::Binary);
    Ok(())
}

```

#### Mesh

```rust
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

```

---

#### License

- MIT