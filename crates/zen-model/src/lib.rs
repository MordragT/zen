//! This crate can deserialize [.mrm](MrmMesh) or [.msh](MshMesh) meshes,
//! and convert them into [gltf] files.
//!
//! ```rust
//! use std::{convert::TryFrom, fs::File, io::Cursor};
//! use zen_archive::Vdfs;
//! use zen_mesh::{gltf, mrm::MrmMesh, Model};
//! use zen_types::path::INSTANCE;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let vdf_file = File::open(INSTANCE.meshes())?;
//! let vdf = Vdfs::new(vdf_file)?;
//! let mesh_entry = vdf
//!     .get_by_name("ORC_MASTERTHRONE.MRM")
//!     .expect("Should be there!");
//! let cursor = Cursor::new(mesh_entry.data);
//! let mesh = MrmMesh::new(cursor, "ORC_MASTERTHRONE")?;
//! let mesh = Model::try_from(mesh)?;
//! let _gltf = gltf::to_gltf(mesh, gltf::Output::Binary);
//! #    Ok(())
//! # }
//! ```

use std::collections::HashMap;
use zen_material::Material;
use zen_math::Vec3;

#[cfg(feature = "gltf")]
#[cfg(feature = "gltf-json")]
pub mod gltf;

pub type Scene = Vec<Model>;

#[repr(C)]
#[derive(Clone, Debug, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

#[derive(Clone, Debug)]
/// Basic Mesh Informations
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material: usize,
    pub num_elements: u32,
}

impl Mesh {
    pub fn extreme_coordinates(&self) -> (Vec3<f32>, Vec3<f32>) {
        self.vertices.iter().fold(
            (
                Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
                Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN),
            ),
            |(mut min, mut max), vertex| {
                let pos = Vec3::from(vertex.position);
                min.min(&pos);
                max.max(&pos);
                (min, max)
            },
        )
    }

    pub fn scale(&mut self, factor: f32) {
        //let origin = self.positions[0];
        for vertex in self.vertices.iter_mut() {
            vertex.position[0] *= factor;
            vertex.position[1] *= factor;
            vertex.position[2] *= factor;
        }
    }

    // TODO not working
    pub fn pack(self) -> Self {
        let Mesh {
            vertices,
            indices,
            material,
            num_elements,
        } = self;
        let (mesh, _) = indices.iter().fold(
            (
                Mesh {
                    vertices: Vec::new(),
                    indices: Vec::new(),
                    material,
                    num_elements,
                },
                HashMap::new(),
            ),
            |(mut mesh, mut map), i| {
                let index = if map.contains_key(i) {
                    *map.get(i).unwrap()
                } else {
                    let idx = *i as usize;

                    let vertex = vertices[idx].clone();
                    mesh.vertices.push(vertex);

                    let len = map.len() as u32;
                    map.insert(*i, len);
                    len as u32
                };
                mesh.indices.push(index);
                (mesh, map)
            },
        );

        mesh
    }
}

#[derive(Clone)]
/// General Mesh consisting out of one or more sub meshes
pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}
