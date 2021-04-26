use error::Error;
use error::Result;
pub use mrm::MrmMesh;
pub use msh::MshMesh;
use vek::Vec3;
//pub use zen::ZenMesh;
use std::convert::{TryFrom, TryInto};
use zen_material::Material;

pub mod error;
pub mod gltf;
pub mod mrm;
pub mod msh;
pub mod structures;
//pub mod zen;

pub struct Mesh {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub tex_coords: Vec<f32>,
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
                    min.x = min.x.min(*pos);
                    max.x = max.x.max(*pos);
                } else if count % 3 == 1 {
                    min.y = min.y.min(*pos);
                    max.y = max.y.max(*pos);
                } else if count % 3 == 2 {
                    min.z = min.z.min(*pos);
                    max.z = max.z.max(*pos);
                }
                (min, max)
            },
        )
    }
}

pub struct SubMesh {
    pub mesh: Mesh,
    pub material: Material,
}

pub struct GeneralMesh {
    pub name: String,
    pub sub_meshes: Vec<SubMesh>,
}

impl TryFrom<MrmMesh> for GeneralMesh {
    type Error = Error;
    fn try_from(object_mesh: MrmMesh) -> Result<Self> {
        let (object_sub_meshes, object_vertices) = (object_mesh.sub_meshes, object_mesh.vertices);
        let sub_meshes = object_sub_meshes
            .into_iter()
            .map(|sub_mesh| {
                let indices = sub_mesh
                    .triangles
                    .into_iter()
                    .flatten()
                    .map(|pos| pos as u32)
                    .collect::<Vec<u32>>();

                let mut mesh = sub_mesh.wedges.into_iter().fold(
                    Mesh {
                        positions: vec![],
                        indices: vec![],
                        normals: vec![],
                        tex_coords: vec![],
                    },
                    |mut mesh, wedge| {
                        mesh.positions
                            .append(&mut object_vertices[wedge.vertex_index as usize].to_vec());
                        mesh.normals.append(&mut wedge.normal.to_vec());
                        mesh.tex_coords.append(&mut wedge.tex_coord.to_vec());
                        mesh
                    },
                );

                mesh.indices = indices;

                let material = (&sub_mesh.material).try_into()?;

                Ok(SubMesh { material, mesh })
            })
            .collect::<Result<Vec<SubMesh>>>()?;
        Ok(Self {
            name: object_mesh.name,
            sub_meshes,
        })
    }
}

impl TryFrom<MshMesh> for GeneralMesh {
    type Error = Error;
    fn try_from(mesh: MshMesh) -> Result<Self> {
        let MshMesh {
            name,
            materials,
            vertices,
            features,
            polygons,
        } = mesh;
        let sub_meshes = polygons
            .into_iter()
            .map(|polygon| -> Result<SubMesh> {
                let verts = polygon
                    .indices
                    .iter()
                    .map(|index| vertices[index.vertex as usize])
                    .flatten()
                    .collect::<Vec<f32>>();
                let norms = polygon
                    .indices
                    .iter()
                    .map(|index| features[index.feature as usize].vert_normal)
                    .flatten()
                    .collect::<Vec<f32>>();
                let tex_coords = polygon
                    .indices
                    .iter()
                    .map(|index| features[index.feature as usize].tex_coord)
                    .flatten()
                    .collect::<Vec<f32>>();
                let indices = (0..verts.len() / 3)
                    .into_iter()
                    .map(|i| i as u32)
                    .collect::<Vec<u32>>();
                Ok(SubMesh {
                    material: (&materials[polygon.material_index as usize]).try_into()?,
                    mesh: Mesh {
                        positions: verts,
                        normals: norms,
                        indices,
                        tex_coords,
                    },
                })
            })
            .collect::<Result<Vec<SubMesh>>>()?;
        Ok(GeneralMesh { name, sub_meshes })
    }
}

// impl From<ZenMesh> for GeneralMesh {
//     fn from(_world_mesh: ZenMesh) -> Self {
//         todo!()
//     }
// }
