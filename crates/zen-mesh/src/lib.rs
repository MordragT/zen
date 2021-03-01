pub use object::ObjectMesh;
pub use scene::SceneMesh;
use vek::Vec3;
use zen_material::Material;

pub mod error;
pub mod gltf;
pub mod object;
pub mod scene;
pub mod world;

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

impl From<ObjectMesh> for GeneralMesh {
    fn from(object_mesh: ObjectMesh) -> Self {
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

                let material = sub_mesh.material.into();

                SubMesh { material, mesh }
            })
            .collect::<Vec<SubMesh>>();
        Self {
            name: object_mesh.name,
            sub_meshes,
        }
    }
}

impl From<SceneMesh> for GeneralMesh {
    fn from(scene_mesh: SceneMesh) -> Self {
        todo!()
    }
}
