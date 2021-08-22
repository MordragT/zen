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

pub use mrm::MrmMesh;
pub use msh::MshMesh;
use zen_math::Vec3;
//pub use zen::ZenMesh;
pub use error::Error;
use error::Result;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};
use zen_material::Material;

mod error;
#[cfg(feature = "gltf")]
#[cfg(feature = "gltf-json")]
pub mod gltf;
pub mod mrm;
pub mod msh;
pub mod structures;
//pub mod zen;

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

// #[derive(Clone)]
// /// Mesh that is a component in another mesh, already holds its material
// pub struct SubMesh {
//     pub mesh: Mesh,
//     pub material: Material,
// }

#[derive(Clone)]
/// General Mesh consisting out of one or more sub meshes
pub struct Model {
    pub name: String,
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

impl TryFrom<MrmMesh> for Model {
    type Error = Error;
    fn try_from(object_mesh: MrmMesh) -> Result<Self> {
        let (object_sub_meshes, object_vertices) = (object_mesh.sub_meshes, object_mesh.vertices);

        let mut materials = Vec::new();
        let meshes = object_sub_meshes
            .into_iter()
            .enumerate()
            .map(|(n, sub_mesh)| {
                let indices = sub_mesh
                    .triangles
                    .into_iter()
                    .map(|v| v.to_array())
                    .flatten()
                    .map(|pos| pos as u32)
                    .collect::<Vec<u32>>();

                let mut mesh = sub_mesh.wedges.into_iter().fold(
                    Mesh {
                        vertices: Vec::new(),
                        num_elements: indices.len() as u32,
                        indices,
                        material: n,
                    },
                    |mut mesh, wedge| {
                        let vertex = Vertex {
                            position: object_vertices[wedge.vertex_index as usize].to_array(),
                            tex_coords: wedge.tex_coord.to_array(),
                            normal: wedge.normal.to_array(),
                        };
                        mesh.vertices.push(vertex);
                        mesh
                    },
                );

                mesh.scale(0.02);

                //let mesh = mesh.pack();
                let material = Material::try_from(&sub_mesh.material)?;
                materials.push(material);

                Ok(mesh)
            })
            .collect::<Result<Vec<Mesh>>>()?;
        Ok(Self {
            name: object_mesh.name,
            meshes,
            materials,
        })
    }
}

impl TryFrom<MshMesh> for Model {
    type Error = Error;
    fn try_from(mesh: MshMesh) -> Result<Self> {
        let MshMesh {
            name,
            materials,
            vertices,
            features,
            polygons,
        } = mesh;
        let mut new_materials = Vec::new();
        let meshes = polygons
            .into_iter()
            .enumerate()
            .map(|(n, polygon)| -> Result<Mesh> {
                let vertices = polygon
                    .indices
                    .iter()
                    .map(|index| Vertex {
                        position: vertices[index.vertex as usize].to_array(),
                        tex_coords: features[index.feature as usize].tex_coord.to_array(),
                        normal: features[index.feature as usize].vert_normal.to_array(),
                    })
                    .collect::<Vec<Vertex>>();
                let indices = (0..vertices.len() / 3)
                    .into_iter()
                    .map(|i| i as u32)
                    .collect::<Vec<u32>>();

                let material = (&materials[polygon.material_index as usize]).try_into()?;
                new_materials.push(material);

                let num_elements = (vertices.len() / 3) as u32;
                Ok(Mesh {
                    vertices,
                    indices,
                    material: n,
                    num_elements,
                })
            })
            .collect::<Result<Vec<Mesh>>>()?;
        Ok(Model {
            name,
            meshes,
            materials: new_materials,
        })
    }
}

// impl From<ZenMesh> for Model {
//     fn from(_world_mesh: ZenMesh) -> Self {
//         todo!()
//     }
// }
