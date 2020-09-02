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

// pub fn to_gltf(input: GeneralMesh, output: Output) -> PathBuf {
//     let mut accessors = vec![]; // positions, colors, normals, indices
//     let mut buffers = vec![]; // vertice_buffer, sub_mesh_buffer
//     let mut buffer_views = vec![]; // vertice_buffer_view, sub_mesh_view
//                                    //let mut primitives = vec![]; // mesh
//     let mut meshes = vec![];
//     let mut nodes = vec![];
//     let mut bin = vec![];
//     let mut length = 0;
//     for (a, (i, (m, sub_mesh))) in input
//         .sub_meshes
//         .into_iter()
//         .enumerate()
//         .enumerate()
//         .step_by(2)
//         .enumerate()
//         .step_by(4)
//     {
//         dbg!(a, i);
//         let triangle_vertices = sub_mesh.vertices;
//         let vertice_buffer_length = (triangle_vertices.len() * mem::size_of::<Vertex>()) as u32;
//         length += vertice_buffer_length;
//         let vertice_buffer = json::Buffer {
//             byte_length: vertice_buffer_length,
//             extensions: Default::default(),
//             extras: Default::default(),
//             name: None,
//             uri: if output == Output::Standard {
//                 Some(format!("buffer{}-0.bin", i))
//             } else {
//                 None
//             },
//         };
//         let vertice_buffer_view = json::buffer::View {
//             buffer: json::Index::new(i as u32),
//             byte_length: vertice_buffer.byte_length,
//             byte_offset: None,
//             byte_stride: Some(mem::size_of::<Vertex>() as u32),
//             extensions: Default::default(),
//             extras: Default::default(),
//             name: None,
//             target: Some(Valid(json::buffer::Target::ArrayBuffer)),
//         };
//         let positions = json::Accessor {
//             buffer_view: Some(json::Index::new(i as u32)),
//             byte_offset: 0,
//             count: triangle_vertices.len() as u32,
//             component_type: Valid(json::accessor::GenericComponentType(
//                 json::accessor::ComponentType::F32,
//             )),
//             extensions: Default::default(),
//             extras: Default::default(),
//             type_: Valid(json::accessor::Type::Vec3),
//             min: Some(json::Value::from(vec![-100f32, -100f32, -100f32])),
//             max: Some(json::Value::from(vec![100f32, 100f32, 100f32])),
//             name: None,
//             normalized: false,
//             sparse: None,
//         };
//         let colors = json::Accessor {
//             buffer_view: Some(json::Index::new(i as u32)),
//             byte_offset: (3 * mem::size_of::<f32>()) as u32,
//             count: triangle_vertices.len() as u32,
//             component_type: Valid(json::accessor::GenericComponentType(
//                 json::accessor::ComponentType::F32,
//             )),
//             extensions: Default::default(),
//             extras: Default::default(),
//             type_: Valid(json::accessor::Type::Vec3),
//             min: None,
//             max: None,
//             name: None,
//             normalized: false,
//             sparse: None,
//         };
//         let normals = json::Accessor {
//             buffer_view: Some(json::Index::new(i as u32)),
//             byte_offset: (6 * mem::size_of::<f32>()) as u32,
//             count: triangle_vertices.len() as u32,
//             component_type: Valid(json::accessor::GenericComponentType(
//                 json::accessor::ComponentType::F32,
//             )),
//             extensions: Default::default(),
//             extras: Default::default(),
//             type_: Valid(json::accessor::Type::Vec3),
//             min: None,
//             max: None,
//             name: None,
//             normalized: false,
//             sparse: None,
//         };

//         let indices = sub_mesh.indices;
//         let indices_length = (indices.len() * mem::size_of::<u32>()) as u32;
//         length += indices_length;
//         let indices_buffer = json::Buffer {
//             byte_length: indices_length,
//             extensions: Default::default(),
//             extras: Default::default(),
//             name: None,
//             uri: if output == Output::Standard {
//                 Some(format!("buffer{}-1.bin", i))
//             } else {
//                 None
//             },
//         };
//         let indices_view = json::buffer::View {
//             buffer: json::Index::new(i as u32 + 1),
//             byte_length: indices_buffer.byte_length,
//             byte_offset: None,
//             byte_stride: None,
//             extensions: Default::default(),
//             extras: Default::default(),
//             name: None,
//             target: Some(Valid(json::buffer::Target::ElementArrayBuffer)),
//         };
//         let indices_acc = json::Accessor {
//             buffer_view: Some(json::Index::new(i as u32 + 1)),
//             byte_offset: 0,
//             count: indices.len() as u32,
//             component_type: Valid(json::accessor::GenericComponentType(
//                 json::accessor::ComponentType::U32,
//             )),
//             extensions: Default::default(),
//             extras: Default::default(),
//             type_: Valid(json::accessor::Type::Scalar),
//             min: None,
//             max: None,
//             name: None,
//             normalized: false,
//             sparse: None,
//         };

//         let primitive = json::mesh::Primitive {
//             attributes: {
//                 let mut map = std::collections::HashMap::new();
//                 map.insert(
//                     Valid(json::mesh::Semantic::Positions),
//                     json::Index::new(a as u32),
//                 );
//                 map.insert(
//                     Valid(json::mesh::Semantic::Colors(0)),
//                     json::Index::new(a as u32 + 1),
//                 );
//                 map.insert(
//                     Valid(json::mesh::Semantic::Normals),
//                     json::Index::new(a as u32 + 2),
//                 );
//                 map
//             },
//             extensions: Default::default(),
//             extras: Default::default(),
//             indices: Some(json::Index::new(a as u32 + 3)),
//             material: None,
//             mode: Valid(json::mesh::Mode::Triangles),
//             targets: None,
//             // targets: Some(vec![json::mesh::MorphTarget {
//             //     positions: Some(json::Index::new(a as u32)),
//             //     normals: None, //Some(json::Index::new(a as u32 + 2)),
//             //     tangents: None,
//             // }]),
//         };
//         let mut padded_vertices = to_padded_byte_vector(triangle_vertices);
//         let mut padded_indices = to_padded_byte_vector(indices);

//         match output {
//             Output::Standard => {
//                 let _ = fs::create_dir("gltf_meshes");
//                 let mut writer =
//                     fs::File::create(format!("gltf_meshes/buffer{}-0.bin", i).as_str())
//                         .expect("I/O error");
//                 writer.write_all(&padded_vertices).expect("I/O error");
//                 let mut writer =
//                     fs::File::create(format!("gltf_meshes/buffer{}-1.bin", i).as_str())
//                         .expect("I/O error");
//                 writer.write_all(&padded_indices).expect("I/O error");
//             }
//             Output::Binary => {
//                 bin.append(&mut padded_vertices);
//                 bin.append(&mut padded_indices);
//             }
//         }

//         buffers.push(vertice_buffer);
//         buffer_views.push(vertice_buffer_view);
//         accessors.push(positions);
//         accessors.push(colors);
//         accessors.push(normals);

//         buffers.push(indices_buffer);
//         buffer_views.push(indices_view);
//         accessors.push(indices_acc);
//         //primitives.push(primitive);

//         let mesh = json::Mesh {
//             extensions: Default::default(),
//             extras: Default::default(),
//             name: None,
//             primitives: vec![primitive],
//             weights: None,
//         };

//         let node = json::Node {
//             camera: None,
//             children: None,
//             extensions: Default::default(),
//             extras: Default::default(),
//             matrix: None,
//             mesh: Some(json::Index::new(m as u32)),
//             name: None,
//             rotation: None,
//             scale: None,
//             translation: None,
//             skin: None,
//             weights: None,
//         };

//         meshes.push(mesh);
//         nodes.push(node);
//     }

//     let mut node_indices = vec![];
//     for (i, _) in nodes.iter().enumerate() {
//         node_indices.push(json::Index::new(i as u32));
//     }

//     let root = json::Root {
//         accessors,
//         buffers,
//         buffer_views,
//         meshes,
//         nodes,
//         scenes: vec![json::Scene {
//             extensions: Default::default(),
//             extras: Default::default(),
//             name: None,
//             nodes: node_indices,
//         }],
//         ..Default::default()
//     };

//     match output {
//         Output::Standard => {
//             let path = PathBuf::from(format!("gltf_meshes/{}.gltf", input.name).as_str());

//             let writer = fs::File::create(&path).expect("I/O error");
//             json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");
//             path
//         }
//         Output::Binary => {
//             let path = PathBuf::from(format!("gltf_meshes/{}.glb", input.name).as_str());
//             let json_string = json::serialize::to_string(&root).expect("Serialization error");
//             let mut json_offset = json_string.len() as u32;
//             align_to_multiple_of_four(&mut json_offset);
//             let glb = gltf::binary::Glb {
//                 header: gltf::binary::Header {
//                     magic: b"glTF".clone(),
//                     version: 2,
//                     length: json_offset + length,
//                 },
//                 bin: Some(Cow::Owned(bin)),
//                 json: Cow::Owned(json_string.into_bytes()),
//             };
//             let writer = std::fs::File::create(&path).expect("I/O error");
//             glb.to_writer(writer).expect("glTF binary output error");
//             path
//         }
//     }
// }
