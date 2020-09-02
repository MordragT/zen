pub use object::ObjectMesh;
pub use scene::SceneMesh;
use std::{cmp, fs, path::Path};
use tri_mesh::prelude::*;
use vek::Vec3;

pub mod error;
pub mod gltf;
pub mod object;
pub mod scene;

// #[derive(Debug, Copy, Clone)]
// pub struct Vertex {
//     pub position: Vec3<f32>,
//     pub color: Vec3<f32>,
//     pub normal: Vec3<f32>,
//     //pub tex_coord: Vec2<f32>,
// }

// pub struct SubMesh {
//     //pub materials: Vec<Material>,
//     pub indices: Vec<u32>,
//     pub positions: Vec<f32>,
//     pub colors: Vec<f32>,
//     pub normals: Vec<f32>,
// }

pub struct GeneralMesh {
    pub name: String,
    pub mesh: Mesh,
    pub colors: Vec<Vec3<f32>>,
}

impl GeneralMesh {
    pub fn to_obj<P: AsRef<Path>>(&self, destination: P) {
        let wavefront = self.mesh.parse_as_obj();
        fs::write(destination, wavefront).unwrap();
    }
}

impl From<ObjectMesh> for GeneralMesh {
    fn from(object_mesh: ObjectMesh) -> Self {
        let mut meshes = vec![];

        for obj_sub_mesh in object_mesh.sub_meshes {
            let mut vertices = vec![];
            let mut indices = vec![];

            for wedge in obj_sub_mesh.wedges {
                vertices.push(object_mesh.vertices[wedge.vertex_index as usize]);
            }
            for triangle in obj_sub_mesh.triangles {
                for position in triangle {
                    indices.push(position as usize);
                }
            }

            let mut positions = vec![];
            for index in indices {
                for pos in vertices[index] {
                    //final_indices.push(final_vertices.len() as u32);
                    positions.push(pos as f64);
                }
            }

            let mut mesh = MeshBuilder::new()
                .with_positions(positions)
                .build()
                .unwrap();
            mesh.fix_orientation();
            mesh.remove_lonely_primitives();
            meshes.push(mesh);
        }
        let mut export_mesh = match meshes.pop() {
            Some(mesh) => mesh,
            None => MeshBuilder::new().cube().build().unwrap(),
        };
        for mesh in meshes {
            export_mesh.append(&mesh);
        }
        Self {
            name: object_mesh.name,
            colors: vec![],
            mesh: export_mesh,
        }
    }
}

fn to_rgb(num: u32) -> Vec3<f32> {
    let layer = |i| {
        cmp::max(
            0,
            cmp::min(1, 3 * i32::abs(1 - 2 * (((num as i32) - i / 3) % 2)) - 1),
        )
    };
    Vec3::new(layer(0) as f32, layer(1) as f32, layer(2) as f32)
}
