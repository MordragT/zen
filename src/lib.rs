//! Rewrite of the [Zenlib](https://github.com/ataulien/ZenLib) in Rust
//!
//! At the moment you can open [VDFS-Archives](zen_archive), and export [Multiresolution-Meshes](zen_mesh::mrm) (.mrm)
//! aswell as normal [Zengin-Meshes](zen_mesh::msh) (.msh) from the archives to gltf files.
//! The corresponding [textures](zen_texture) will also be exported (similiar to dds textures),
//! or you can export those textures one by one aswell.
//!
//! I am working on the export of Zengin World Meshes (.zen) to gltf
//! and a [Daedalus](zen_daedalus) (scripting language) virtual machine to execute the bytecode.

pub use zen_core;
pub use zen_daedalus;
pub use zen_parser;

mod prelude {
    // pub use zen_archive::{Entry, Vdfs};
    // pub use zen_parser::ascii::de::AsciiDeserializer;
}
