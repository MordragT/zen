use super::GeneralMesh;
use gltf_json as json;
use json::validation::Checked::Valid;
use std::borrow::Cow;
use std::{
    fs,
    io::Write,
    mem,
    path::{Path, PathBuf},
};
use zen_types::path::FILES_INSTANCE;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Output {
    /// Output standard glTF.
    Standard,

    /// Output binary glTF.
    Binary,
}

fn align_to_multiple_of_four(n: &mut u32) {
    *n = (*n + 3) & !3;
}

fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * mem::size_of::<T>();
    let byte_capacity = vec.capacity() * mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

const NUM: u32 = 4;

pub fn to_gltf(input: GeneralMesh, output: Output) -> PathBuf {
    let mut accessors = vec![]; //positions, indices, normals],
    let mut buffers = vec![];
    let mut buffer_views = vec![]; //positions_view, indices_view, normals_view],
    let mut primitives = vec![];
    let mut images = vec![];
    let mut textures = vec![];
    let mut materials = vec![];

    let mut buffer_length = 0;
    let mut bin = vec![];

    let length = if output == Output::Binary {
        let mut byte_length = 0;
        for sub_mesh in input.sub_meshes.iter() {
            byte_length += (sub_mesh.mesh.positions.len() * mem::size_of::<f32>()) as u32;
            byte_length += (sub_mesh.mesh.indices.len() * mem::size_of::<u32>()) as u32;
            byte_length += (sub_mesh.mesh.normals.len() * mem::size_of::<f32>()) as u32;
            byte_length += (sub_mesh.mesh.tex_coords.len() * mem::size_of::<f32>()) as u32;
            // byte_length +=
            //     (dbg!(sub_mesh.material.texture.get_ref().len()) * mem::size_of::<u8>()) as u32;
        }
        let buffer = json::Buffer {
            byte_length,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            uri: None,
        };
        buffers.push(buffer);
        byte_length
    } else {
        0
    };

    for (i, sub_mesh) in input.sub_meshes.into_iter().enumerate() {
        let bound = sub_mesh.mesh.extreme_coordinates();
        let positions_vec = sub_mesh.mesh.positions;
        let indices_vec = sub_mesh.mesh.indices;
        let normals_vec = sub_mesh.mesh.normals;
        let tex_coords_vec = sub_mesh.mesh.tex_coords;

        let positions_buffer_length = (positions_vec.len() * mem::size_of::<f32>()) as u32;
        let indices_buffer_length = (indices_vec.len() * mem::size_of::<u32>()) as u32;
        let normals_buffer_length = (normals_vec.len() * mem::size_of::<f32>()) as u32;
        let tex_coords_buffer_length = (tex_coords_vec.len() * mem::size_of::<f32>()) as u32;

        let positions_view = json::buffer::View {
            buffer: if output == Output::Standard {
                json::Index::new(i as u32 * NUM)
            } else {
                json::Index::new(0)
            },
            byte_length: positions_buffer_length,
            byte_offset: if output == Output::Binary {
                Some(buffer_length)
            } else {
                None
            },
            byte_stride: None,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        };
        buffer_views.push(positions_view);

        let positions = json::Accessor {
            buffer_view: Some(json::Index::new(i as u32 * NUM)),
            byte_offset: 0,
            count: positions_vec.len() as u32 / 3,
            component_type: Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec3),
            min: Some(json::Value::from(bound.0.to_vec())),
            max: Some(json::Value::from(bound.1.to_vec())),
            name: None,
            normalized: false,
            sparse: None,
        };
        accessors.push(positions);

        let indices_view = json::buffer::View {
            buffer: if output == Output::Standard {
                json::Index::new(i as u32 * NUM + 1)
            } else {
                json::Index::new(0)
            },
            byte_length: indices_buffer_length,
            byte_offset: if output == Output::Binary {
                Some(buffer_length + positions_buffer_length)
            } else {
                None
            },
            byte_stride: None,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            target: Some(Valid(json::buffer::Target::ElementArrayBuffer)),
        };
        buffer_views.push(indices_view);

        let indices = json::Accessor {
            buffer_view: Some(json::Index::new(i as u32 * NUM + 1)),
            byte_offset: 0,
            count: indices_vec.len() as u32,
            component_type: Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::U32,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Scalar),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
        };
        accessors.push(indices);

        let normals_view = json::buffer::View {
            buffer: if output == Output::Standard {
                json::Index::new(i as u32 * NUM + 2)
            } else {
                json::Index::new(0)
            },
            byte_length: normals_buffer_length,
            byte_offset: if output == Output::Binary {
                Some(buffer_length + positions_buffer_length + indices_buffer_length)
            } else {
                None
            },
            byte_stride: None,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        };
        buffer_views.push(normals_view);

        let normals = json::Accessor {
            buffer_view: Some(json::Index::new(i as u32 * NUM + 2)),
            byte_offset: 0,
            count: normals_vec.len() as u32 / 3,
            component_type: Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec3),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
        };
        accessors.push(normals);

        let tex_coords_view = json::buffer::View {
            buffer: if output == Output::Standard {
                json::Index::new(i as u32 * NUM + 3)
            } else {
                json::Index::new(0)
            },
            byte_length: tex_coords_buffer_length,
            byte_offset: if output == Output::Binary {
                Some(
                    buffer_length
                        + positions_buffer_length
                        + indices_buffer_length
                        + normals_buffer_length,
                )
            } else {
                None
            },
            byte_stride: None,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        };
        buffer_views.push(tex_coords_view);

        let tex_coords = json::Accessor {
            buffer_view: Some(json::Index::new(i as u32 * NUM + 3)),
            byte_offset: 0,
            count: tex_coords_vec.len() as u32 / 2,
            component_type: Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec2),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
        };
        accessors.push(tex_coords);

        let image = json::Image {
            name: None,
            buffer_view: None, //Some(json::Index::new(i as u32 * NUM + 4)),
            mime_type: Some(json::image::MimeType("image/jpeg".to_string())),
            uri: Some(
                Path::new("../")
                    .join(
                        sub_mesh
                            .material
                            .texture
                            .strip_prefix(&FILES_INSTANCE.base_path)
                            .unwrap(),
                    )
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
            extensions: None,
            extras: Default::default(),
        };
        images.push(image);

        let texture = json::Texture {
            name: None,
            sampler: None,
            source: json::Index::new(i as u32),
            extensions: None,
            extras: Default::default(),
        };
        textures.push(texture);

        let material = json::Material {
            alpha_cutoff: json::material::AlphaCutoff(0.0),
            alpha_mode: Valid(json::material::AlphaMode::Mask),
            pbr_metallic_roughness: json::material::PbrMetallicRoughness {
                base_color_texture: Some(json::texture::Info {
                    index: json::Index::new(i as u32),
                    tex_coord: 0,
                    extensions: None,
                    extras: Default::default(),
                }),
                metallic_factor: json::material::StrengthFactor(0.0),
                ..Default::default()
            },
            ..Default::default()
        };
        materials.push(material);

        let primitive = json::mesh::Primitive {
            attributes: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    Valid(json::mesh::Semantic::Positions),
                    json::Index::new(i as u32 * NUM),
                );
                map.insert(
                    Valid(json::mesh::Semantic::Normals),
                    json::Index::new(i as u32 * NUM + 2),
                );
                map.insert(
                    Valid(json::mesh::Semantic::TexCoords(0)),
                    json::Index::new(i as u32 * NUM + 3),
                );
                map
            },
            extensions: Default::default(),
            extras: Default::default(),
            indices: Some(json::Index::new(i as u32 * NUM + 1)),
            material: Some(json::Index::new(i as u32)),
            mode: Valid(json::mesh::Mode::Triangles),
            targets: None,
        };
        primitives.push(primitive);

        let mut inner_buffers = match output {
            Output::Standard => {
                let positions_buffer = json::Buffer {
                    byte_length: positions_buffer_length,
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: None,
                    uri: Some(format!("{}-positions-{}.bin", input.name, i)),
                };
                let positions = to_padded_byte_vector(positions_vec);
                let mut writer = fs::File::create(
                    FILES_INSTANCE
                        .meshes
                        .join(format!("{}-positions-{}.bin", input.name, i)),
                )
                .expect("I/O error");
                writer.write_all(&positions).expect("I/O error");

                let indices_buffer = json::Buffer {
                    byte_length: indices_buffer_length,
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: None,
                    uri: Some(format!("{}-indices-{}.bin", input.name, i)),
                };
                let indices = to_padded_byte_vector(indices_vec);
                let mut writer = fs::File::create(
                    FILES_INSTANCE
                        .meshes
                        .join(format!("{}-indices-{}.bin", input.name, i)),
                )
                .expect("I/O error");
                writer.write_all(&indices).expect("I/O error");

                let normals_buffer = json::Buffer {
                    byte_length: normals_buffer_length,
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: None,
                    uri: Some(format!("{}-normals-{}.bin", input.name, i)),
                };
                let normals = to_padded_byte_vector(normals_vec);
                let mut writer = fs::File::create(
                    FILES_INSTANCE
                        .meshes
                        .join(format!("{}-normals-{}.bin", input.name, i)),
                )
                .expect("I/O error");
                writer.write_all(&normals).expect("I/O error");

                let tex_coords_buffer = json::Buffer {
                    byte_length: tex_coords_buffer_length,
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: None,
                    uri: Some(format!("{}-tex_coords-{}.bin", input.name, i)),
                };
                let tex_coords = to_padded_byte_vector(tex_coords_vec);
                let mut writer = fs::File::create(
                    FILES_INSTANCE
                        .meshes
                        .join(format!("{}-tex_coords-{}.bin", input.name, i)),
                )
                .expect("I/O error");
                writer.write_all(&tex_coords).expect("I/O error");

                // let textures_buffer = json::Buffer {
                //     byte_length: texture_buffer_length,
                //     extensions: Default::default(),
                //     extras: Default::default(),
                //     name: None,
                //     uri: Some(format!("{}-textures-{}.bin", input.name, i)),
                // };
                // let textures = to_padded_byte_vector(textures_vec);
                // let mut writer =
                //     fs::File::create(format!("mesh/{}-textures-{}.bin", input.name, i))
                //         .expect("I/O error");
                // writer.write_all(&textures).expect("I/O error");

                vec![
                    positions_buffer,
                    indices_buffer,
                    normals_buffer,
                    tex_coords_buffer,
                    //textures_buffer,
                ]
            }
            Output::Binary => {
                bin.append(&mut to_padded_byte_vector(positions_vec));
                bin.append(&mut to_padded_byte_vector(indices_vec));
                bin.append(&mut to_padded_byte_vector(normals_vec));
                bin.append(&mut to_padded_byte_vector(tex_coords_vec));
                //bin.append(&mut to_padded_byte_vector(textures_vec));
                vec![]
            }
        };
        buffers.append(&mut inner_buffers);

        buffer_length += positions_buffer_length;
        buffer_length += indices_buffer_length;
        buffer_length += normals_buffer_length;
        buffer_length += tex_coords_buffer_length;
        //buffer_length += texture_buffer_length;
    }

    let mesh = json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives,
        weights: None,
    };

    let node = json::Node {
        camera: None,
        children: None,
        extensions: Default::default(),
        extras: Default::default(),
        matrix: None,
        mesh: Some(json::Index::new(0)),
        name: None,
        rotation: None,
        scale: None,
        translation: None,
        skin: None,
        weights: None,
    };

    let root = json::Root {
        accessors,
        buffers,
        buffer_views,
        meshes: vec![mesh],
        nodes: vec![node],
        scenes: vec![json::Scene {
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            nodes: vec![json::Index::new(0)],
        }],
        images,
        textures,
        materials,
        ..Default::default()
    };

    match output {
        Output::Standard => {
            let path = PathBuf::from(FILES_INSTANCE.meshes.join(format!("{}.gltf", input.name)));
            let writer = fs::File::create(&path).expect("I/O error");
            json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");
            path
        }
        Output::Binary => {
            let path = PathBuf::from(FILES_INSTANCE.meshes.join(format!("{}.glb", input.name)));
            let json_string = json::serialize::to_string(&root).expect("Serialization error");
            let mut json_offset = json_string.len() as u32;
            align_to_multiple_of_four(&mut json_offset);
            let glb = gltf::binary::Glb {
                header: gltf::binary::Header {
                    magic: b"glTF".clone(),
                    version: 2,
                    length: json_offset + length,
                },
                bin: Some(Cow::Owned(bin)),
                json: Cow::Owned(json_string.into_bytes()),
            };
            let writer = std::fs::File::create(&path).expect("I/O error");
            glb.to_writer(writer).expect("glTF binary output error");
            path
        }
    }
}
