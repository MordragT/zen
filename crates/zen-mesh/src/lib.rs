pub use object::ObjectMesh;
pub use scene::SceneMesh;
use std::{cmp, fs, io::Cursor, path::Path};
use tri_mesh::prelude::*;
use vek::Vec2;
use zen_material::Material;

pub mod error;
pub mod gltf;
pub mod object;
pub mod scene;

pub struct SubMesh {
    pub mesh: Mesh,
    pub material: Material,
    pub tex_coords: Vec<f32>,
}

pub struct GeneralMesh {
    pub name: String,
    pub sub_meshes: Vec<SubMesh>,
}

// impl GeneralMesh {
//     pub fn to_obj<P: AsRef<Path>>(&self, destination: P) {
//         let wavefront = self.mesh.parse_as_obj();
//         fs::write(destination, wavefront).unwrap();
//     }
// }

impl From<ObjectMesh> for GeneralMesh {
    fn from(object_mesh: ObjectMesh) -> Self {
        let mut sub_meshes = vec![];

        for sub_mesh in object_mesh.sub_meshes {
            let mut vertices = vec![];
            let mut indices = vec![];
            let mut tex_coords = vec![];

            for wedge in sub_mesh.wedges {
                vertices.push(object_mesh.vertices[wedge.vertex_index as usize]);
                tex_coords.push(wedge.tex_coord);
            }
            for triangle in sub_mesh.triangles {
                for position in triangle {
                    indices.push(position as usize);
                }
            }

            let material = sub_mesh.material.into();

            let mut positions = vec![];
            let mut final_tex_coords = vec![];
            for index in indices {
                for pos in vertices[index] {
                    //final_indices.push(final_vertices.len() as u32);
                    positions.push(pos as f64);
                }
                for tex_coord in tex_coords[index] {
                    final_tex_coords.push(tex_coord);
                }
            }

            let mesh = MeshBuilder::new()
                .with_positions(positions)
                .build()
                .unwrap();
            //mesh.fix_orientation();
            //mesh.remove_lonely_primitives();

            let sub_mesh = SubMesh {
                material,
                mesh,
                tex_coords: final_tex_coords,
            };
            sub_meshes.push(sub_mesh);
        }
        Self {
            name: object_mesh.name,
            sub_meshes,
        }
    }
}
