use crate::assets::ZenLoadContext;
use crate::material::ZenMaterial;
use crate::texture::{TextureError, ZenTexture};

use super::ZenModel;
use super::{Vertex, ZenMesh};
use bevy::prelude::Assets;
use gltf_json as json;
use gltf_json::validation::USize64;
use image::codecs::png::PngEncoder;
use json::validation::Checked::Valid;
use std::borrow::Cow;
use std::{
    fs::File,
    io::{self, Write},
    mem,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Output {
    /// Output standard glTF.
    Standard(PathBuf),

    /// Output binary glTF.
    Binary(PathBuf),
}

impl Output {
    pub fn standard<P: AsRef<Path>>(path: P) -> Self {
        Self::Standard(path.as_ref().to_owned())
    }

    pub fn binary<P: AsRef<Path>>(path: P) -> Self {
        Self::Binary(path.as_ref().to_owned())
    }
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

#[derive(Debug, Error)]
pub enum GltfError {
    #[error("Io: {0}")]
    Io(#[from] io::Error),
    #[error("Texture: {0}")]
    Texture(#[from] TextureError),
    #[error("Serialization: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Gltf: {0}")]
    Gltf(#[from] gltf::Error),
}

type GltfResult<T> = Result<T, GltfError>;

#[derive(Debug)]
pub struct GltfBuilder {
    root: json::Root,
    buffer: Vec<u8>,
}

impl GltfBuilder {
    pub fn new() -> Self {
        let mut root = json::Root::default();

        root.buffers.push(json::Buffer {
            byte_length: USize64(0),
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            uri: None,
        });

        let buffer = Vec::new();

        Self { root, buffer }
    }

    pub fn build_primitive(&mut self, mesh: ZenMesh) -> PrimitiveBuilder {
        let (min, max) = mesh.extreme_coordinates();
        let ZenMesh { vertices, indices } = mesh;
        let num_elements = vertices.len() as u64;

        PrimitiveBuilder { primitive }
    }

    pub fn build_mesh(&mut self) -> MeshBuilder<'_> {
        let mesh = json::Mesh {
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            primitives: Vec::new(),
            weights: None,
        };

        MeshBuilder {
            builder: self,
            mesh,
        }
    }

    pub fn build_model(
        &mut self,
        model: ZenModel,
        context: &mut ZenLoadContext,
    ) -> GltfResult<NodeBuilder<'_>> {
        let ZenLoadContext {
            meshes,
            materials,
            textures,
        } = context;

        let mut empty = json::Node {
            camera: None,
            children: None,
            extensions: Default::default(),
            extras: Default::default(),
            matrix: None,
            mesh: None,
            name: None,
            rotation: None,
            scale: None,
            translation: None,
            skin: None,
            weights: None,
        };

        let children = model
            .into_iter()
            .filter_map(|model| {
                let ZenModel {
                    name,
                    mesh,
                    material,
                    transform,
                    ..
                } = model;

                if let Some(mesh_handle) = mesh {
                    let mesh = meshes.remove(&mesh_handle).expect("Mesh was not loaded");
                    Some((name, mesh, material, transform))
                } else {
                    None
                }
            })
            .map(|(name, mesh, material, transform)| {
                let mut material_index = None;
                if let Some(material_handle) = material {
                    let material = materials
                        .remove(&material_handle)
                        .expect("Material was not loaded");
                    material_index = Some(self.build_material(material, *textures)?.build());
                }

                let mut primitive = self.build_primitive(mesh);

                primitive = if let Some(index) = material_index {
                    primitive.set_material(index)
                } else {
                    primitive
                };

                let primitive = primitive.create();
                let mesh = self.build_mesh().add_primitive(primitive).build();
                let mut node = empty.clone();
                node.mesh = Some(mesh);
                node.name = Some(name);
                node.translation = Some(transform.translation.to_array());
                node.rotation = Some(json::scene::UnitQuaternion(transform.rotation.to_array()));

                let index = json::Index::new(self.root.nodes.len() as u32);
                self.root.nodes.push(node);

                Ok(index)
            })
            .collect::<GltfResult<_>>()?;

        empty.children = Some(children);

        Ok(NodeBuilder {
            builder: self,
            node: empty,
        })
    }

    pub fn build_scene(&mut self) -> SceneBuilder<'_> {
        let scene = json::Scene {
            extensions: Default::default(),
            extras: Default::default(),
            name: None,
            nodes: Vec::new(),
        };

        SceneBuilder {
            builder: self,
            scene,
        }
    }

    pub fn build(mut self, scene: json::Index<json::Scene>, output: Output) -> GltfResult<()> {
        self.root.scene = Some(scene);

        match output {
            Output::Binary(at) => {
                let json_string = json::serialize::to_string(&self.root)?;
                let mut json_offset = json_string.len() as u32;
                align_to_multiple_of_four(&mut json_offset);
                let glb = gltf::binary::Glb {
                    header: gltf::binary::Header {
                        magic: b"glTF".clone(),
                        version: 2,
                        length: json_offset + self.buffer.len() as u32,
                    },
                    bin: Some(Cow::Owned(self.buffer)),
                    json: Cow::Owned(json_string.into_bytes()),
                };
                let writer = File::create(at)?;
                glb.to_writer(writer)?;
            }
            Output::Standard(at) => {
                let mut buffer_writer = File::create(format!("{at:?}.bin"))?;
                buffer_writer.write_all(&mut self.buffer)?;

                let writer = File::create(at)?;
                json::serialize::to_writer_pretty(writer, &self.root)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PrimitiveBuilder {
    primitive: json::mesh::Primitive,
}

impl PrimitiveBuilder {
    pub fn set_material(mut self, material: json::Index<json::Material>) -> Self {
        self.primitive.material = Some(material);
        self
    }

    pub fn create(self) -> json::mesh::Primitive {
        self.primitive
    }
}

#[derive(Debug)]
pub struct MeshBuilder<'a> {
    builder: &'a mut GltfBuilder,
    mesh: json::Mesh,
}

impl<'a> MeshBuilder<'a> {
    pub fn add_primitive(mut self, primitive: json::mesh::Primitive) -> Self {
        self.mesh.primitives.push(primitive);
        self
    }

    pub fn build(self) -> json::Index<json::Mesh> {
        let index = json::Index::new(self.builder.root.meshes.len() as u32);
        self.builder.root.meshes.push(self.mesh);
        index
    }
}

#[derive(Debug)]
pub struct NodeBuilder<'a> {
    builder: &'a mut GltfBuilder,
    node: json::Node,
}

impl<'a> NodeBuilder<'a> {
    pub fn add_mesh(self, mesh: json::Index<json::Mesh>) -> Self {
        let node = json::Node {
            camera: None,
            children: None,
            extensions: Default::default(),
            extras: Default::default(),
            matrix: None,
            mesh: Some(mesh),
            name: None,
            rotation: None,
            scale: None,
            translation: None,
            skin: None,
            weights: None,
        };
        let index = json::Index::new(self.builder.root.nodes.len() as u32);
        self.builder.root.nodes.push(node);

        self.add_node(index)
    }

    pub fn add_node(mut self, node: json::Index<json::Node>) -> Self {
        match self.node.children {
            Some(ref mut children) => children.push(node),
            None => unreachable!(),
        }

        self
    }

    pub fn build(self) -> json::Index<json::Node> {
        let index = json::Index::new(self.builder.root.nodes.len() as u32);
        self.builder.root.nodes.push(self.node);
        index
    }
}

#[derive(Debug)]
pub struct MaterialBuilder<'a> {
    builder: &'a mut GltfBuilder,
    material: json::Material,
}

impl<'a> MaterialBuilder<'a> {
    pub fn set_texture(mut self, texture: json::Index<json::Texture>) -> Self {
        match self.material.pbr_metallic_roughness.base_color_texture {
            Some(ref mut info) => info.index = texture,
            None => unreachable!(),
        }
        self
    }

    pub fn set_normal_texture(mut self, texture: json::Index<json::Texture>) -> Self {
        self.material.normal_texture = Some(json::material::NormalTexture {
            index: texture,
            scale: 1.0,
            tex_coord: 0,
            extensions: None,
            extras: None,
        });
        self
    }

    pub fn set_occlusion_texture(mut self, texture: json::Index<json::Texture>) -> Self {
        self.material.occlusion_texture = Some(json::material::OcclusionTexture {
            index: texture,
            tex_coord: 0,
            strength: json::material::StrengthFactor::default(),
            extensions: None,
            extras: None,
        });
        self
    }

    pub fn set_emissive_texture(mut self, texture: json::Index<json::Texture>) -> Self {
        self.material.emissive_texture = Some(json::texture::Info {
            index: texture,
            tex_coord: 0,
            extensions: None,
            extras: None,
        });
        self
    }

    pub fn build(self) -> json::Index<json::Material> {
        let index = json::Index::new(self.builder.root.materials.len() as u32);
        self.builder.root.materials.push(self.material);
        index
    }
}

#[derive(Debug)]
pub struct SceneBuilder<'a> {
    builder: &'a mut GltfBuilder,
    scene: json::Scene,
}

impl<'a> SceneBuilder<'a> {
    pub fn add_node(mut self, node: json::Index<json::Node>) -> Self {
        self.scene.nodes.push(node);
        self
    }

    pub fn build(self) -> json::Index<json::Scene> {
        let index = json::Index::new(self.builder.root.scenes.len() as u32);
        self.builder.root.scenes.push(self.scene);
        index
    }
}

// TODO fix Output::Binary

impl ZenModel {
    pub fn to_gltf(self, context: &mut ZenLoadContext, output: Output) -> GltfResult<()> {
        let mut builder = GltfBuilder::new();
        let node = builder.build_model(self, context)?.build();
        let scene = builder.build_scene().add_node(node).build();

        builder.build(scene, output)
    }
}
