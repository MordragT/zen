use error::*;
use gltf_json as json;
use json::validation::Checked::Valid;
use serde::Deserialize;
use std::borrow::Cow;
use std::io::{Seek, SeekFrom, Write};
use std::{fs, mem, path::PathBuf};
use zen_parser::prelude::*;
use zen_types::{material, mesh};

pub mod error;

const MESH: u16 = 0xB000;
const BBOX3D: u16 = 0xB010;
const MAT_LIST: u16 = 0xB020;
const LIGHT_MAP_LIST: u16 = 0xB025;
const LIGHT_MAP_LIST_SHARED: u16 = 0xB026;
const VERT_LIST: u16 = 0xB030;
const FEAT_LIST: u16 = 0xB040;
const POLY_LIST: u16 = 0xB050;
const MESH_END: u16 = 0xB060;

const PROG_MESH: u16 = 45312;
const PROG_MESH_END: u16 = 45567;

const GOTHIC2_6: u32 = 265;
const GOTHIC1_08K: u32 = 9;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Output {
    /// Output standard glTF.
    Standard,

    /// Output binary glTF.
    Binary,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GltfVertex {
    position: [f32; 3],
    color: [f32; 3],
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
pub struct Mesh {
    name: String,
    vertices: Vec<(f32, f32, f32)>,
    polygons: Option<Vec<mesh::Polygon>>,
    sub_meshes: Option<Vec<mesh::SubMesh>>,
}

impl Mesh {
    pub fn from_world<'a, R: BinaryRead + AsciiRead>(reader: R) -> Result<Mesh> {
        let mut deserializer = BinaryDeserializer::from(reader);

        // min, max
        //let mut bounding_box: ((f32, f32, f32), (f32, f32, f32));
        let mut name = String::new();
        let mut vertices = vec![];
        let mut _features = vec![];
        let mut polygons = vec![];
        let mut materials = vec![];
        let mut version = 0;

        loop {
            let chunk = mesh::Chunk::deserialize(&mut deserializer)?;
            let chunk_end = SeekFrom::Current(chunk.length as i64);
            //println!("id:{} -> length: {}", chunk.id, chunk.length);
            match chunk.id {
                MESH => {
                    #[derive(Deserialize)]
                    struct Info {
                        version: u32,
                        date: mesh::Date,
                        name: String,
                    }
                    let info = Info::deserialize(&mut deserializer)?;
                    println!("Reading mesh {} with version: {}", info.name, info.version);
                    version = info.version;
                    name = info.name;
                    deserializer.seek(chunk_end)?;
                }
                BBOX3D => {
                    println!("Reading bounding box");
                    let (_min, _max) = <((f32, f32, f32, f32), (f32, f32, f32, f32))>::deserialize(
                        &mut deserializer,
                    )?;
                    //bounding_box = ((min.0, min.1, min.2), (max.0, max.1, max.2));
                    deserializer.seek(chunk_end)?;
                }
                MAT_LIST => {
                    println!("Reading material list");
                    let mut ascii_deserializer = AsciiDeserializer::from(deserializer);
                    ascii_deserializer.read_header()?;
                    deserializer = ascii_deserializer.into();

                    let material_num = u32::deserialize(&mut deserializer)?;
                    for _ in 0..material_num {
                        let material: material::Material = {
                            let _name = String::deserialize(&mut deserializer)?;
                            // Skip name and chunk headers
                            let material_header =
                                material::ChunkHeader::deserialize(&mut deserializer)?;

                            // Skip chunk header
                            let _name = String::deserialize(&mut deserializer)?;
                            let _class_name = String::deserialize(&mut deserializer)?;

                            // Save into Vec
                            match material_header.version {
                                material::GOTHIC2 => {
                                    material::BasicMaterial::deserialize(&mut deserializer)?.into()
                                }
                                _ => material::AdvancedMaterial::deserialize(&mut deserializer)?
                                    .into(),
                            }
                        };
                        materials.push(material);
                    }
                    deserializer.seek(chunk_end)?;
                }
                LIGHT_MAP_LIST => {
                    println!("Reading light map list");
                    deserializer.seek(chunk_end)?;
                }
                LIGHT_MAP_LIST_SHARED => {
                    println!("Reading light map list");
                    deserializer.seek(chunk_end)?;
                }
                VERT_LIST => {
                    println!("Reading vertice list");
                    let num_vertices = u32::deserialize(&mut deserializer)?;
                    deserializer.len_queue.push(num_vertices as usize);
                    vertices = <Vec<(f32, f32, f32)>>::deserialize(&mut deserializer)?;
                }
                FEAT_LIST => {
                    println!("Reading feature list");
                    let num_feats = u32::deserialize(&mut deserializer)?;
                    deserializer.len_queue.push(num_feats as usize);
                    _features = <Vec<mesh::FeatureChunk>>::deserialize(&mut deserializer)?;
                }
                POLY_LIST => {
                    println!("Reading polygon list");
                    let num_polys = u32::deserialize(&mut deserializer)?;
                    for _ in 0..num_polys {
                        // let data_block_seed = deserializer.seek(SeekFrom::Current(0))?;
                        // deserializer.seek(chunk_end);

                        // TODO: nochmal in referenz gucken, deserialzation geschieht erst nachher

                        #[repr(packed(1))]
                        #[derive(Deserialize)]
                        struct PolygonData {
                            pub material_index: i16,
                            pub light_map_index: i16,
                            pub plane: mesh::PlanePacked,
                        }
                        let polygon_data = PolygonData::deserialize(&mut deserializer)?;

                        let flags: mesh::PolyFlags = match version {
                            GOTHIC2_6 => {
                                <mesh::PolyGothicTwoFlags>::deserialize(&mut deserializer)?.into()
                            }
                            GOTHIC1_08K => todo!(),
                            _ => return Err(Error::UnknownGameVersion),
                        };

                        let num_indices = u8::deserialize(&mut deserializer)?;

                        let indices_result = (0..num_indices)
                            .map(|_| {
                                let index: mesh::Index = match version {
                                    GOTHIC2_6 => {
                                        <mesh::IndexPacked<u32>>::deserialize(&mut deserializer)?
                                            .into()
                                    }
                                    GOTHIC1_08K => {
                                        <mesh::IndexPacked<u16>>::deserialize(&mut deserializer)?
                                            .into()
                                    }
                                    _ => return Err(Error::UnknownGameVersion),
                                };
                                return Ok(index);
                            })
                            .collect::<Result<Vec<mesh::Index>>>();

                        let indices = match indices_result {
                            Ok(i) => i,
                            Err(s) => return Err(s),
                        };

                        let polygon = mesh::Polygon::new(
                            polygon_data.material_index,
                            polygon_data.light_map_index,
                            polygon_data.plane.into(),
                            flags,
                            num_indices,
                            indices,
                        );
                        polygons.push(polygon);
                    }
                    deserializer.seek(chunk_end)?;
                }
                MESH_END => break,
                _ => {
                    deserializer.seek(chunk_end)?;
                }
            }
        }
        Ok(Self {
            name,
            vertices,
            polygons: Some(polygons),
            sub_meshes: None,
        })
    }

    pub fn from_mrm<R: BinaryRead + AsciiRead>(reader: R) -> Result<Mesh> {
        let mut deserializer = BinaryDeserializer::from(reader);

        // min, max
        //let mut bounding_box: ((f32, f32, f32), (f32, f32, f32));
        //let mut name = String::new();
        let mut vertices = vec![];
        let mut sub_meshes = vec![];
        //let mut version = 0;

        let chunk = <mesh::Chunk>::deserialize(&mut deserializer)?;
        let chunk_end = SeekFrom::Current(chunk.length as i64);
        //println!("id:{} -> length: {}", chunk.id, chunk.length);
        match chunk.id {
            PROG_MESH => {
                let _version = u16::deserialize(&mut deserializer)?;

                let data_size = u32::deserialize(&mut deserializer)?;
                println!("Mesh data size {}", data_size);
                let data_seek = deserializer.seek(SeekFrom::Current(0))?;
                println!("Mesh data seek {}", data_seek);
                deserializer.seek(SeekFrom::Current(data_size as i64))?;

                let num_sub_meshes = u8::deserialize(&mut deserializer)?;
                let main_offsets = <mesh::Offset>::deserialize(&mut deserializer)?;

                deserializer.len_queue.push(num_sub_meshes as usize);
                let sub_mesh_offsets = <Vec<mesh::SubMeshOffsets>>::deserialize(&mut deserializer)?;

                //let header = binary::read_header(&mut deserializer)?;
                let mut ascii_de = AsciiDeserializer::from(deserializer);
                ascii_de.read_header()?;
                deserializer = ascii_de.into();

                // for b in deserializer.by_ref().take(500).bytes() {
                //     dbg!(b.unwrap() as char);
                // }

                let mut materials = (0..num_sub_meshes)
                    .map(|_| {
                        let material: material::Material = {
                            let _name = String::deserialize(&mut deserializer)?;
                            // Skip name and chunk headers
                            let material_header =
                                material::ChunkHeader::deserialize(&mut deserializer)?;

                            // Skip chunk header
                            let _name = String::deserialize(&mut deserializer)?;
                            let _class_name = String::deserialize(&mut deserializer)?;

                            // Save into Vec
                            match material_header.version {
                                material::GOTHIC2 => {
                                    material::AdvancedMaterial::deserialize(&mut deserializer)?
                                        .into()
                                }
                                _ => {
                                    material::BasicMaterial::deserialize(&mut deserializer)?.into()
                                }
                            }
                        };
                        // let mut buf = [0; 200];
                        // deserializer.parser.read_exact(&mut buf)?;
                        // dbg!(String::from_utf8_lossy(&buf));
                        Ok(material)
                    })
                    .collect::<Result<Vec<material::Material>>>()?;

                // TODO gothic 1 should not read byte
                let _alpha_test = u8::deserialize(&mut deserializer)?;

                // bounding box
                let (_min, _max) =
                    <((f32, f32, f32, f32), (f32, f32, f32, f32))>::deserialize(&mut deserializer)?;

                deserializer.seek(SeekFrom::Start(
                    data_seek + main_offsets.position.offset as u64,
                ))?;

                deserializer
                    .len_queue
                    .push(main_offsets.position.size as usize);
                vertices = <Vec<(f32, f32, f32)>>::deserialize(&mut deserializer)?;

                for offset in sub_mesh_offsets {
                    let triangles = match offset.triangles.offset > data_size {
                        true => {
                            return Err(Error::Message(format!(
                                "Internal error, offset {} out of bound {}",
                                offset.triangles.offset, data_size
                            )))
                        }
                        false => {
                            deserializer.seek(SeekFrom::Start(
                                data_seek + offset.triangles.offset as u64,
                            ))?;
                            deserializer.len_queue.push(offset.triangles.size as usize);
                            <Vec<mesh::Triangle>>::deserialize(&mut deserializer)?
                        }
                    };
                    deserializer.seek(SeekFrom::Start(data_seek + offset.wedges.offset as u64))?;
                    deserializer.len_queue.push(offset.wedges.size as usize);
                    let wedges = <Vec<mesh::Wedge>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(data_seek + offset.colors.offset as u64))?;
                    deserializer.len_queue.push(offset.colors.size as usize);
                    let colors = <Vec<f32>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(
                        data_seek + offset.triangle_plane_indices.offset as u64,
                    ))?;
                    deserializer
                        .len_queue
                        .push(offset.triangle_plane_indices.size as usize);
                    let triangle_plane_indices = <Vec<u16>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(
                        data_seek + offset.triangle_planes.offset as u64,
                    ))?;
                    deserializer
                        .len_queue
                        .push(offset.triangle_planes.size as usize);
                    let triangle_planes = <Vec<mesh::Plane>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(
                        data_seek + offset.triangle_edges.offset as u64,
                    ))?;
                    deserializer
                        .len_queue
                        .push(offset.triangle_edges.size as usize);
                    let triangle_edges = <Vec<(u16, u16, u16)>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(data_seek + offset.edges.offset as u64))?;
                    deserializer.len_queue.push(offset.edges.size as usize);
                    let edges = <Vec<(u16, u16)>>::deserialize(&mut deserializer)?;

                    deserializer.seek(SeekFrom::Start(
                        data_seek + offset.edge_scores.offset as u64,
                    ))?;
                    deserializer
                        .len_queue
                        .push(offset.edge_scores.size as usize);
                    let edge_scores = <Vec<f32>>::deserialize(&mut deserializer)?;

                    deserializer
                        .seek(SeekFrom::Start(data_seek + offset.wedge_map.offset as u64))?;
                    deserializer.len_queue.push(offset.wedge_map.size as usize);
                    let wedge_map = <Vec<u16>>::deserialize(&mut deserializer)?;

                    let sub_mesh = mesh::SubMesh::new(
                        materials.remove(0),
                        triangles,
                        wedges,
                        colors,
                        triangle_plane_indices,
                        triangle_planes,
                        triangle_edges,
                        wedge_map,
                        edges,
                        edge_scores,
                    );

                    sub_meshes.push(sub_mesh);
                }
                deserializer.seek(chunk_end)?;
            }
            _ => {
                return Err(Error::ExpectedIdentifier(format!(
                    "PROG_MESH: {}",
                    PROG_MESH
                )))
            }
        }
        Ok(Self {
            name: String::new(),
            vertices,
            polygons: None,
            sub_meshes: Some(sub_meshes),
        })
    }
    pub fn to_gltf(self, output: Output) -> PathBuf {
        // let triangle_vertices = vec![
        //     GltfVertex {
        //         position: [0.0, 0.5, 0.0],
        //         color: [1.0, 0.0, 0.0],
        //     },
        //     GltfVertex {
        //         position: [-0.5, -0.5, 0.0],
        //         color: [0.0, 1.0, 0.0],
        //     },
        //     GltfVertex {
        //         position: [0.5, -0.5, 0.0],
        //         color: [0.0, 0.0, 1.0],
        //     },
        // ];
        let triangle_vertices: Vec<GltfVertex> = self
            .vertices
            .into_iter()
            .map(|v| {
                return GltfVertex {
                    position: [v.0, v.1, v.2],
                    color: [1.0, 0.0, 0.0],
                };
            })
            .collect();

        let buffer_length = (triangle_vertices.len() * mem::size_of::<GltfVertex>()) as u32;
        let buffer = json::Buffer {
            byte_length: buffer_length,
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            uri: if output == Output::Standard {
                Some("buffer0.bin".into())
            } else {
                None
            },
        };
        let buffer_view = json::buffer::View {
            buffer: json::Index::new(0),
            byte_length: buffer.byte_length,
            byte_offset: None,
            byte_stride: Some(mem::size_of::<GltfVertex>() as u32),
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            target: Some(Valid(json::buffer::Target::ArrayBuffer)),
        };
        let positions = json::Accessor {
            buffer_view: Some(json::Index::new(0)),
            byte_offset: 0,
            count: triangle_vertices.len() as u32,
            component_type: Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            extensions: Default::default(),
            extras: Default::default(),
            type_: Valid(json::accessor::Type::Vec3),
            min: Some(json::Value::from(vec![-0.5f32, -0.5f32, 0.0f32])),
            max: Some(json::Value::from(vec![0.5f32, 0.5f32, 0.0f32])),
            name: None,
            normalized: false,
            sparse: None,
        };
        let colors = json::Accessor {
            buffer_view: Some(json::Index::new(0)),
            byte_offset: (3 * mem::size_of::<f32>()) as u32,
            count: triangle_vertices.len() as u32,
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
                map.insert(Valid(json::mesh::Semantic::Colors(0)), json::Index::new(1));
                map
            },
            extensions: Default::default(),
            extras: Default::default(),
            indices: None,
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
            accessors: vec![positions, colors],
            buffers: vec![buffer],
            buffer_views: vec![buffer_view],
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
                let _ = fs::create_dir("gltf_meshes");
                let path = PathBuf::from(format!("gltf_meshes/{}.gltf", self.name).as_str());

                let writer = fs::File::create(&path).expect("I/O error");
                json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

                let bin = to_padded_byte_vector(triangle_vertices);
                let mut writer =
                    fs::File::create(format!("gltf_meshes/{}_buffer.bin", self.name).as_str())
                        .expect("I/O error");
                writer.write_all(&bin).expect("I/O error");
                path
            }
            Output::Binary => {
                let path = PathBuf::from(format!("gltf_meshes/{}.glb", self.name).as_str());
                let json_string = json::serialize::to_string(&root).expect("Serialization error");
                let mut json_offset = json_string.len() as u32;
                align_to_multiple_of_four(&mut json_offset);
                let glb = gltf::binary::Glb {
                    header: gltf::binary::Header {
                        magic: b"glTF".clone(),
                        version: 2,
                        length: json_offset + buffer_length,
                    },
                    bin: Some(Cow::Owned(to_padded_byte_vector(triangle_vertices))),
                    json: Cow::Owned(json_string.into_bytes()),
                };
                let writer = std::fs::File::create(&path).expect("I/O error");
                glb.to_writer(writer).expect("glTF binary output error");
                path
            }
        }
    }
}
