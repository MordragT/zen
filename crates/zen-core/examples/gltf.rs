use zen_material::Material;
use zen_math::Vec3;
use zen_model::{gltf, Mesh, Model, Vertex};
use zen_texture::Texture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = Model {
        name: "name".to_owned(),
        materials: vec![Material {
            texture: Texture::new(
                2,
                2,
                zen_texture::ColorType::RGBA8,
                vec![
                    1, 2, 3, 0xff, 5, 6, 7, 0xff, 9, 10, 11, 0xff, 13, 14, 15, 0xff,
                ],
                "texture".to_owned(),
            ),
            color: Vec3::new(0.1, 0.2, 0.3),
        }],
        meshes: vec![
            Mesh {
                vertices: vec![
                    Vertex {
                        position: [0.0, 0.0, 0.0],
                        tex_coords: [0.0, 0.0],
                        normal: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, 0.0, 0.0],
                        tex_coords: [1.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, 1.0, 0.0],
                        tex_coords: [0.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [1.0, 1.0, 0.0],
                        tex_coords: [1.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                ],
                indices: vec![0, 1, 2, 3, 2, 1],
                material: 0,
                num_elements: 4,
            },
            Mesh {
                vertices: vec![
                    Vertex {
                        position: [2.0, 2.0, 2.0],
                        tex_coords: [0.0, 0.0],
                        normal: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [2.0, 3.0, 2.0],
                        tex_coords: [1.0, 0.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [2.0, 2.0, 3.0],
                        tex_coords: [0.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [2.0, 3.0, 3.0],
                        tex_coords: [1.0, 1.0],
                        normal: [0.0, 0.0, 0.0],
                    },
                ],
                indices: vec![0, 1, 2, 3, 2, 1],
                material: 0,
                num_elements: 4,
            },
        ],
    };

    let _gltf = gltf::to_gltf(model.clone(), gltf::Output::Standard);
    let _gltf = gltf::to_gltf(model, gltf::Output::Binary);

    Ok(())
}
