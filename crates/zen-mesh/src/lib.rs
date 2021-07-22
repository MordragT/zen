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

#[derive(Clone)]
/// Basic Mesh Informations
pub struct Mesh {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub tex_coords: Vec<f32>,
    pub material: usize,
    pub num_elements: u32,
}

impl Mesh {
    pub fn extreme_coordinates(&self) -> (Vec3<f32>, Vec3<f32>) {
        self.positions.iter().enumerate().fold(
            (
                Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
                Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN),
            ),
            |(mut min, mut max), (count, pos)| {
                if count % 3 == 0 {
                    min.min_x(*pos);
                    max.max_x(*pos);
                } else if count % 3 == 1 {
                    min.min_y(*pos);
                    max.max_y(*pos);
                } else if count % 3 == 2 {
                    min.min_z(*pos);
                    max.max_z(*pos);
                }
                (min, max)
            },
        )
    }

    // TODO not working
    pub fn pack(self) -> Self {
        let Mesh {
            indices,
            positions,
            normals,
            tex_coords,
            material,
            num_elements,
        } = self;
        let (mesh, _) = indices.iter().fold(
            (
                Mesh {
                    positions: Vec::new(),
                    indices: Vec::new(),
                    normals: Vec::new(),
                    tex_coords: Vec::new(),
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

                    let vertex = &positions[idx..idx + 3];
                    assert!(vertex.len() == 3);
                    mesh.positions.extend_from_slice(vertex);

                    let normal = &normals[idx..idx + 3];
                    assert!(normal.len() == 3);
                    mesh.normals.extend_from_slice(normal);

                    let tex_coord = &tex_coords[idx..idx + 2];
                    assert!(tex_coord.len() == 2);
                    mesh.tex_coords.extend_from_slice(tex_coord);

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

                let mesh = sub_mesh.wedges.into_iter().fold(
                    Mesh {
                        positions: vec![],
                        indices,
                        normals: vec![],
                        tex_coords: vec![],
                        material: n,
                        num_elements: 0,
                    },
                    |mut mesh, wedge| {
                        mesh.positions
                            .append(&mut object_vertices[wedge.vertex_index as usize].to_vec());
                        mesh.num_elements += 1;
                        mesh.normals.append(&mut wedge.normal.to_vec());
                        mesh.tex_coords.append(&mut wedge.tex_coord.to_vec());
                        mesh
                    },
                );

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
                let verts = polygon
                    .indices
                    .iter()
                    .map(|index| vertices[index.vertex as usize].to_array())
                    .flatten()
                    .collect::<Vec<f32>>();
                let norms = polygon
                    .indices
                    .iter()
                    .map(|index| features[index.feature as usize].vert_normal.to_array())
                    .flatten()
                    .collect::<Vec<f32>>();
                let tex_coords = polygon
                    .indices
                    .iter()
                    .map(|index| features[index.feature as usize].tex_coord.to_array())
                    .flatten()
                    .collect::<Vec<f32>>();
                let indices = (0..verts.len() / 3)
                    .into_iter()
                    .map(|i| i as u32)
                    .collect::<Vec<u32>>();

                let material = (&materials[polygon.material_index as usize]).try_into()?;
                new_materials.push(material);

                let num_elements = (verts.len() / 3) as u32;
                Ok(Mesh {
                    positions: verts,
                    normals: norms,
                    indices,
                    tex_coords,
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
