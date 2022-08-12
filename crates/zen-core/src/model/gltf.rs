use super::ZenModel;
use super::{Vertex, ZenMesh};
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
    let byte_length = vec.len() * std::mem::size_of::<T>();
    let byte_capacity = vec.capacity() * std::mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

/// There are 2 buffers: vertices and indices
const BUFFER_NUM: usize = 2;
const BUFFER_VIEW_NUM: usize = 2;

// TODO fix Output::Biary

pub fn to_gltf(input: ZenModel, output: Output) -> PathBuf {
    struct RootInformation {
        bin: Vec<u8>,
        buffers: Vec<json::Buffer>,
        buffer_views: Vec<json::buffer::View>,
        primitives: Vec<json::mesh::Primitive>,
        accessors: Vec<json::Accessor>,
        textures: Vec<json::Texture>,
        images: Vec<json::Image>,
        materials: Vec<json::Material>,
    }

    impl RootInformation {
        pub fn new() -> Self {
            Self {
                bin: Vec::new(),
                buffers: Vec::new(),
                buffer_views: Vec::new(),
                primitives: Vec::new(),
                accessors: Vec::new(),
                textures: Vec::new(),
                images: Vec::new(),
                materials: Vec::new(),
            }
        }
    }

    let ZenModel {
        meshes,
        materials,
        name,
    } = input;

    let info =
        meshes
            .into_iter()
            .enumerate()
            .fold(RootInformation::new(), |mut info, (num, mesh)| {
                let (min, max) = mesh.extreme_coordinates();

                let ZenMesh {
                    vertices, indices, ..
                } = mesh;

                let vertices_buffer_len = mesh.num_elements * mem::size_of::<Vertex>() as u32;
                let indices_buffer_len = (indices.len() * mem::size_of::<u32>()) as u32;

                let vertices_view = json::buffer::View {
                    buffer: json::Index::new((BUFFER_NUM * num) as u32),
                    byte_length: vertices_buffer_len,
                    byte_offset: None,
                    byte_stride: Some(mem::size_of::<Vertex>() as u32),
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: None,
                    target: Some(Valid(json::buffer::Target::ArrayBuffer)),
                };
                info.buffer_views.push(vertices_view);

                let positions = json::Accessor {
                    buffer_view: Some(json::Index::new((BUFFER_VIEW_NUM * num) as u32)),
                    byte_offset: 0,
                    count: mesh.num_elements,
                    component_type: Valid(json::accessor::GenericComponentType(
                        json::accessor::ComponentType::F32,
                    )),
                    extensions: Default::default(),
                    extras: Default::default(),
                    type_: Valid(json::accessor::Type::Vec3),
                    min: Some(json::Value::from(min.to_array().to_vec())),
                    max: Some(json::Value::from(max.to_array().to_vec())),
                    name: None,
                    normalized: false,
                    sparse: None,
                };
                info.accessors.push(positions);

                let normals = json::Accessor {
                    buffer_view: Some(json::Index::new((BUFFER_VIEW_NUM * num) as u32)),
                    byte_offset: (3 * mem::size_of::<f32>()) as u32,
                    count: mesh.num_elements,
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
                info.accessors.push(normals);

                let tex_coords = json::Accessor {
                    buffer_view: Some(json::Index::new((BUFFER_VIEW_NUM * num) as u32)),
                    byte_offset: (6 * mem::size_of::<f32>()) as u32,
                    count: mesh.num_elements,
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
                info.accessors.push(tex_coords);

                let indices_view = json::buffer::View {
                    buffer: json::Index::new(((BUFFER_NUM * num) + 1) as u32),
                    byte_length: indices_buffer_len,
                    byte_offset: None,
                    byte_stride: None,
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: None,
                    target: Some(Valid(json::buffer::Target::ElementArrayBuffer)),
                };
                info.buffer_views.push(indices_view);

                let indices_accessor = json::Accessor {
                    buffer_view: Some(json::Index::new(((BUFFER_VIEW_NUM * num) + 1) as u32)),
                    byte_offset: 0,
                    count: indices.len() as u32,
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
                info.accessors.push(indices_accessor);

                let mut image_name = materials[mesh.material]
                    .texture
                    .name
                    .split('.')
                    .next()
                    .expect("Name should have been validated before in zen-material!")
                    .to_owned();
                image_name.push_str(".png");
                let image_path = FILES_INSTANCE.textures.join(&image_name);
                // TODO: remove unwrap
                let image_output = fs::File::create(&image_path).unwrap();
                materials[mesh.material]
                    .texture
                    .to_png(image_output)
                    .unwrap();

                let image = json::Image {
                    name: None,
                    buffer_view: None, //Some(json::Index::new(i as u32 * NUM + 4)),
                    mime_type: Some(json::image::MimeType("image/png".to_owned())),
                    uri: Some(
                        Path::new("../")
                            .join(image_path.strip_prefix(&FILES_INSTANCE.base_path).unwrap())
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ),
                    extensions: None,
                    extras: Default::default(),
                };
                info.images.push(image);

                let texture = json::Texture {
                    name: None,
                    sampler: None,
                    source: json::Index::new(num as u32),
                    extensions: None,
                    extras: Default::default(),
                };
                info.textures.push(texture);

                let material = json::Material {
                    alpha_cutoff: Some(json::material::AlphaCutoff(0.0)),
                    alpha_mode: Valid(json::material::AlphaMode::Mask),
                    pbr_metallic_roughness: json::material::PbrMetallicRoughness {
                        base_color_texture: Some(json::texture::Info {
                            index: json::Index::new(num as u32),
                            tex_coord: 0,
                            extensions: None,
                            extras: Default::default(),
                        }),
                        metallic_factor: json::material::StrengthFactor(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                info.materials.push(material);

                let primitive = json::mesh::Primitive {
                    attributes: {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            Valid(json::mesh::Semantic::Positions),
                            json::Index::new((num * 4) as u32),
                        );
                        map.insert(
                            Valid(json::mesh::Semantic::Normals),
                            json::Index::new((num as u32 * 4) + 1),
                        );
                        map.insert(
                            Valid(json::mesh::Semantic::TexCoords(0)),
                            json::Index::new((num as u32 * 4) + 2),
                        );
                        map
                    },
                    extensions: Default::default(),
                    extras: Default::default(),
                    indices: Some(json::Index::new((num as u32 * 4) + 3)),
                    material: Some(json::Index::new(num as u32)),
                    mode: Valid(json::mesh::Mode::Triangles),
                    targets: None,
                };
                info.primitives.push(primitive);

                // let padded_vertices_buffer_len = {
                //     let mut padded = vertices_buffer_len;
                //     while padded % 4 != 0 {
                //         padded += 1;
                //     }
                //     padded
                // };

                let vertices_buffer = json::Buffer {
                    byte_length: vertices_buffer_len,
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: None,
                    uri: match output {
                        Output::Binary => None,
                        Output::Standard => Some(format!("{}-vertices-{}.bin", name, num)),
                    },
                };
                info.buffers.push(vertices_buffer);

                // let padded_indices_buffer_len = {
                //     let mut padded = indices_buffer_len;
                //     while padded % 4 != 0 {
                //         padded += 1;
                //     }
                //     padded
                // };

                let indices_buffer = json::Buffer {
                    byte_length: indices_buffer_len,
                    extensions: Default::default(),
                    extras: Default::default(),
                    name: None,
                    uri: match output {
                        Output::Binary => None,
                        Output::Standard => Some(format!("{}-indices-{}.bin", name, num)),
                    },
                };
                info.buffers.push(indices_buffer);

                match output {
                    Output::Binary => {
                        info.bin.append(&mut to_padded_byte_vector(vertices));
                        info.bin.append(&mut to_padded_byte_vector(indices));
                    }
                    Output::Standard => {
                        let vertices = to_padded_byte_vector(vertices);
                        let mut writer = fs::File::create(
                            FILES_INSTANCE
                                .meshes
                                .join(format!("{}-vertices-{}.bin", name, num)),
                        )
                        .expect("I/O error");
                        writer.write_all(&vertices).expect("I/O error");

                        let indices = to_padded_byte_vector(indices);
                        let mut writer = fs::File::create(
                            FILES_INSTANCE
                                .meshes
                                .join(format!("{}-indices-{}.bin", name, num)),
                        )
                        .expect("I/O error");
                        writer.write_all(&indices).expect("I/O error");
                    }
                };
                info
            });

    let RootInformation {
        bin,
        buffers,
        buffer_views,
        primitives,
        accessors,
        textures,
        images,
        materials,
    } = info;

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
            let path = PathBuf::from(FILES_INSTANCE.meshes.join(format!("{}.gltf", name)));
            let writer = fs::File::create(&path).expect("I/O error");
            json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");
            path
        }
        Output::Binary => {
            let path = PathBuf::from(FILES_INSTANCE.meshes.join(format!("{}.glb", name)));
            let json_string = json::serialize::to_string(&root).expect("Serialization error");
            let mut json_offset = json_string.len() as u32;
            align_to_multiple_of_four(&mut json_offset);
            let glb = gltf::binary::Glb {
                header: gltf::binary::Header {
                    magic: b"glTF".clone(),
                    version: 2,
                    length: json_offset + bin.len() as u32,
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
