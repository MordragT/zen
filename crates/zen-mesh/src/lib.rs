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

// impl From<ObjectMesh> for GeneralMesh {
//     fn from(mesh: ObjectMesh) -> Self {
//         let mut vertices = vec![];
//         dbg!(mesh.sub_meshes.len());
//         for sub_mesh in mesh.sub_meshes {
//             //dbg!(sub_mesh.clone());
//             // Wedges aus sub_mesh laden
//             for wedge in sub_mesh.wedges {
//                 //dbg!(wedge.vertex_index);
//                 match mesh.vertices.get(wedge.vertex_index as usize) {
//                     Some(vertex) => {
//                         vertices.push(*vertex); // == vxs.push(v.position)
//                     }
//                     None => {
//                         dbg!(wedge.vertex_index); // Sollte niemals vorkommen
//                     }
//                 };
//             }
//         }
//         Self {
//             name: String::new(),
//             mesh: MeshBuilder::new().cube().build().unwrap(),
//             colors: vec![],
//         }
//     }
// }

fn to_rgb(num: u32) -> Vec3<f32> {
    let layer = |i| {
        cmp::max(
            0,
            cmp::min(1, 3 * i32::abs(1 - 2 * (((num as i32) - i / 3) % 2)) - 1),
        )
    };
    Vec3::new(layer(0) as f32, layer(1) as f32, layer(2) as f32)
}

impl From<ObjectMesh> for GeneralMesh {
    fn from(object_mesh: ObjectMesh) -> Self {
        let mut meshes = vec![];
        let mut vertices = vec![];
        let mut indices = vec![];

        dbg!(std::mem::size_of::<zen_types::mesh::object::Wedge>());

        for obj_sub_mesh in object_mesh.sub_meshes {
            // zenlib hier vertice_start = vertices.len()
            let vertice_start = vertices.len();

            for wedge in obj_sub_mesh.wedges {
                vertices.push(object_mesh.vertices[wedge.vertex_index as usize]);
                // match object_mesh.vertices.get(wedge.vertex_index as usize) {
                //     Some(vertex) => {
                //         vertices.push(*vertex); // == vxs.push(v.position)
                //     }
                //     None => {
                //         //
                //     }
                // };
            }
            // alle vertices die für das submesh benötigt werden sind in vertices
            // jedoch gehen in indices alle indices von allen submeshes

            let obj_sub_mesh_triangles_len = obj_sub_mesh.triangles.len();

            let index_start = indices.len();
            for triangle in obj_sub_mesh.triangles {
                for position in triangle {
                    indices.push(position as usize + vertice_start);
                }
            }

            let mut sub_mesh_indices = vec![];
            for i in index_start..index_start + obj_sub_mesh_triangles_len * 3 {
                match indices.get(i) {
                    Some(index) => sub_mesh_indices.push(*index),
                    None => {
                        dbg!(2);
                    }
                }
            }

            // sub_mesh_indices zeigt nun auf die richtigen einträge in vertices
            // vertices müssen allerdings submesh spezifisch sein
            // und vertices werden zu eindimensionaler lister verändert
            // dadurch müssen auch die indices weieder verändert werden

            let mut final_indices = vec![];
            let mut final_vertices = vec![];
            for index in sub_mesh_indices {
                match vertices.get(index) {
                    Some(vertex) => {
                        for e in vertex {
                            final_indices.push(final_vertices.len() as u32);
                            final_vertices.push(*e as f64);
                        }
                    }
                    None => {
                        dbg!(3);
                    }
                }
            }

            // nun zeigen mega viele indices auf verschiedene vertices obwohl diese die gleiche position
            // besitzen, bisschen schwund ist immer ;)

            // TODO dieses hässliche konstrukt verbessern

            //println!("Positions: {:?}", sub_mesh_pos);

            let mut mesh = MeshBuilder::new()
                .with_positions(final_vertices)
                //.with_indices(final_indices)
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
