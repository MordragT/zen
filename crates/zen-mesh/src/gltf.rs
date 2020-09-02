use super::GeneralMesh;
use gltf_json as json;
use json::validation::Checked::Valid;
use std::borrow::Cow;
use std::{fs, io::Write, mem, path::PathBuf};

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

pub fn to_gltf(input: GeneralMesh, output: Output) -> PathBuf {
    let positions_vec = input.mesh.positions_buffer_f32();
    let indices_vec = input.mesh.indices_buffer();
    let normals_vec = input.mesh.normals_buffer_f32();
    let bound = input.mesh.extreme_coordinates();

    let positions_buffer_length = (positions_vec.len() * mem::size_of::<f32>()) as u32;
    let indices_buffer_length = (indices_vec.len() * mem::size_of::<u32>()) as u32;
    let normals_buffer_length = (normals_vec.len() * mem::size_of::<f32>()) as u32;

    let buffers = match output {
        Output::Standard => {
            let positions_buffer = json::Buffer {
                byte_length: positions_buffer_length,
                extensions: Default::default(),
                extras: Default::default(),
                name: None,
                uri: Some(format!("{}-buffer0.bin", input.name)),
            };

            let indices_buffer = json::Buffer {
                byte_length: indices_buffer_length,
                extensions: Default::default(),
                extras: Default::default(),
                name: None,
                uri: Some(format!("{}-buffer1.bin", input.name)),
            };

            let normals_buffer = json::Buffer {
                byte_length: normals_buffer_length,
                extensions: Default::default(),
                extras: Default::default(),
                name: None,
                uri: Some(format!("{}-buffer2.bin", input.name)),
            };
            vec![positions_buffer, indices_buffer, normals_buffer]
        }
        Output::Binary => {
            let buffer = json::Buffer {
                byte_length: positions_buffer_length
                    + indices_buffer_length
                    + normals_buffer_length,
                extensions: Default::default(),
                extras: Default::default(),
                name: None,
                uri: None,
            };
            vec![buffer]
        }
    };

    let positions_view = json::buffer::View {
        buffer: json::Index::new(0),
        byte_length: positions_buffer_length,
        byte_offset: None,
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    };
    let positions = json::Accessor {
        buffer_view: Some(json::Index::new(0)),
        byte_offset: 0,
        count: positions_vec.len() as u32 / 3,
        component_type: Valid(json::accessor::GenericComponentType(
            json::accessor::ComponentType::F32,
        )),
        extensions: Default::default(),
        extras: Default::default(),
        type_: Valid(json::accessor::Type::Vec3),
        min: Some(json::Value::from(vec![bound.0.x, bound.0.y, bound.0.z])),
        max: Some(json::Value::from(vec![bound.1.x, bound.1.y, bound.1.z])),
        name: None,
        normalized: false,
        sparse: None,
    };

    let indices_view = json::buffer::View {
        buffer: if output == Output::Standard {
            json::Index::new(1)
        } else {
            json::Index::new(0)
        },
        byte_length: indices_buffer_length,
        byte_offset: if output == Output::Binary {
            Some(positions_buffer_length)
        } else {
            None
        },
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ElementArrayBuffer)),
    };
    let indices = json::Accessor {
        buffer_view: Some(json::Index::new(1)),
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

    let normals_view = json::buffer::View {
        buffer: if output == Output::Standard {
            json::Index::new(2)
        } else {
            json::Index::new(0)
        },
        byte_length: normals_buffer_length,
        byte_offset: if output == Output::Binary {
            Some(positions_buffer_length + indices_buffer_length)
        } else {
            None
        },
        byte_stride: None,
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        target: Some(Valid(json::buffer::Target::ArrayBuffer)),
    };
    let normals = json::Accessor {
        buffer_view: Some(json::Index::new(2)),
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

    let primitive = json::mesh::Primitive {
        attributes: {
            let mut map = std::collections::HashMap::new();
            map.insert(Valid(json::mesh::Semantic::Positions), json::Index::new(0));
            map.insert(Valid(json::mesh::Semantic::Normals), json::Index::new(2));
            map
        },
        extensions: Default::default(),
        extras: Default::default(),
        indices: Some(json::Index::new(1)),
        material: None,
        mode: Valid(json::mesh::Mode::Triangles),
        targets: None,
    };

    let mesh = json::Mesh {
        extensions: Default::default(),
        extras: Default::default(),
        name: None,
        primitives: vec![primitive],
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
        accessors: vec![positions, indices, normals],
        buffers,
        buffer_views: vec![positions_view, indices_view, normals_view],
        meshes: vec![mesh],
        nodes: vec![node],
        scenes: vec![json::Scene {
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            nodes: vec![json::Index::new(0)],
        }],
        ..Default::default()
    };

    match output {
        Output::Standard => {
            let _ = fs::create_dir("mesh");
            let path = PathBuf::from(format!("mesh/{}.gltf", input.name));
            let writer = fs::File::create(&path).expect("I/O error");
            json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

            let positions = to_padded_byte_vector(positions_vec);
            let mut writer =
                fs::File::create(format!("mesh/{}-buffer0.bin", input.name)).expect("I/O error");
            writer.write_all(&positions).expect("I/O error");
            let indices = to_padded_byte_vector(indices_vec);
            let mut writer =
                fs::File::create(format!("mesh/{}-buffer1.bin", input.name)).expect("I/O error");
            writer.write_all(&indices).expect("I/O error");
            let normals = to_padded_byte_vector(normals_vec);
            let mut writer =
                fs::File::create(format!("mesh/{}-buffer2.bin", input.name)).expect("I/O error");
            writer.write_all(&normals).expect("I/O error");
            path
        }
        Output::Binary => {
            let _ = fs::create_dir("mesh");
            let path = PathBuf::from(format!("mesh/{}.glb", input.name));
            let json_string = json::serialize::to_string(&root).expect("Serialization error");
            let mut json_offset = json_string.len() as u32;
            align_to_multiple_of_four(&mut json_offset);
            let mut bin = to_padded_byte_vector(positions_vec);
            bin.append(&mut to_padded_byte_vector(indices_vec));
            bin.append(&mut to_padded_byte_vector(normals_vec));
            let glb = gltf::binary::Glb {
                header: gltf::binary::Header {
                    magic: b"glTF".clone(),
                    version: 2,
                    length: json_offset
                        + positions_buffer_length
                        + indices_buffer_length
                        + normals_buffer_length,
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
