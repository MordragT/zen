//! This crate holds data structures required for [Model] objects
//! and different operations on them.

use crate::{material::ZenMaterial, math::Vec3};
use bevy::{
    ecs::system::SystemParamItem,
    prelude::Handle,
    prelude::{
        Bundle, Component, ComputedVisibility, GlobalTransform, MaterialMeshBundle, Res, Transform,
        Visibility,
    },
    reflect::{TypeUuid, Uuid},
    render::{
        extract_component::ExtractComponent,
        mesh::{
            GpuBufferInfo, GpuMesh, Indices, InnerMeshVertexBufferLayout, Mesh,
            MeshVertexBufferLayout, PrimitiveTopology,
        },
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            BufferInitDescriptor, BufferUsages, IndexFormat, VertexAttribute, VertexBufferLayout,
            VertexStepMode,
        },
        renderer::RenderDevice,
    },
};
use std::collections::{HashMap, VecDeque};
use std::mem;

#[cfg(feature = "gltf")]
#[cfg(feature = "gltf-json")]
pub mod gltf;

#[repr(C)]
#[derive(Clone, Debug, Copy, bytemuck::Pod, bytemuck::Zeroable)]
/// A simple Vertex
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

// #[derive(Bundle, Clone, TypeUuid, Default)]
// #[uuid = "2e393245-9977-43a8-97f2-2a0d54700b9d"]
// pub struct ZenModelBundle {
//     pub model: Handle<ZenModel>,
//     pub transform: Transform,
//     pub global_transform: GlobalTransform,
//     pub visibility: Visibility,
//     pub computed_visibility: ComputedVisibility,
// }

/// General Mesh consisting out of one or more sub meshes
#[derive(Clone, Debug, TypeUuid)]
#[uuid = "bf78b0e2-3835-11ed-a261-0242ac120002"]
pub struct ZenModel {
    pub name: String,
    pub children: Vec<ZenModel>,
    pub mesh: Option<Handle<ZenMesh>>,
    pub material: Option<Handle<ZenMaterial>>,
    pub transform: Transform,
}

impl IntoIterator for ZenModel {
    type Item = ZenModel;
    type IntoIter = ZenModelIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        ZenModelIntoIterator { queue }
    }
}

pub struct ZenModelIntoIterator {
    queue: VecDeque<ZenModel>,
}

impl Iterator for ZenModelIntoIterator {
    type Item = ZenModel;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut item) = self.queue.pop_front() {
            if let Some(child) = item.children.pop() {
                self.queue.push_back(child);
                self.queue.push_back(item);
                self.next()
            } else {
                Some(item)
            }
        } else {
            None
        }
    }
}

// impl ExtractComponent for ZenModel {
//     type Query = &'static Self;
//     type Filter = ();

//     fn extract_component(item: bevy::ecs::query::QueryItem<Self::Query>) -> Self {
//         item.clone()
//     }
// }

// #[derive(Bundle, Clone, TypeUuid, Default)]
// #[uuid = "4bb17946-383a-11ed-a261-0242ac120002"]
// pub struct ZenMeshBundle {
//     pub mesh: Handle<ZenMesh>,
//     pub transform: Transform,
//     pub global_transform: GlobalTransform,
//     pub visibility: Visibility,
//     pub computed_visibility: ComputedVisibility,
// }

/// Basic Mesh Informations
#[derive(Clone, Debug, TypeUuid)]
#[uuid = "88834d9b-44d4-4686-a570-5cfdd66052b5"]
pub struct ZenMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

// TODO impl Into BevyMesh instead of RenderAsset

impl From<ZenMesh> for Mesh {
    fn from(mesh: ZenMesh) -> Self {
        let primitive_topology = PrimitiveTopology::TriangleList;
        let mut bevy_mesh = Mesh::new(primitive_topology);

        let len = mesh.vertices.len();
        let mut positions = Vec::with_capacity(len);
        let mut normals = Vec::with_capacity(len);
        let mut uvs = Vec::with_capacity(len);

        for vertex in mesh.vertices {
            positions.push(vertex.position);
            normals.push(vertex.normal);
            uvs.push(vertex.tex_coords);
        }

        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        bevy_mesh.set_indices(Some(Indices::U32(mesh.indices)));

        bevy_mesh
    }
}

// impl RenderAsset for ZenMesh {
//     type ExtractedAsset = Mesh;
//     type PreparedAsset = GpuMesh;
//     type Param = Res<'static, RenderDevice>;

//     fn extract_asset(&self) -> Self::ExtractedAsset {
//         log::debug!("Extracting ZenMesh for GPU");
//         self.clone().into()
//     }

//     fn prepare_asset(
//         mesh: Self::ExtractedAsset,
//         render_device: &mut SystemParamItem<Self::Param>,
//     ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
//         log::debug!("Preparing ZenMesh for GPU");
//         <Mesh as RenderAsset>::prepare_asset(mesh, render_device)
//     }
// }

impl ZenMesh {
    pub fn plane(size: f32) -> Self {
        let vertices = vec![
            Vertex {
                position: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [size, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, size],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [size, 0.0, size],
                tex_coords: [1.0, 1.0],
                normal: [1.0, 0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 1, 3, 2];
        Self { vertices, indices }
    }

    pub fn extreme_coordinates(&self) -> (Vec3<f32>, Vec3<f32>) {
        self.vertices.iter().fold(
            (
                Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX),
                Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN),
            ),
            |(mut min, mut max), vertex| {
                let pos = Vec3::from(vertex.position);
                min.min(&pos);
                max.max(&pos);
                (min, max)
            },
        )
    }

    pub fn scale(&mut self, factor: f32) {
        //let origin = self.positions[0];
        for vertex in self.vertices.iter_mut() {
            vertex.position[0] *= factor;
            vertex.position[1] *= factor;
            vertex.position[2] *= factor;
        }
    }

    // TODO not working
    pub fn pack(self) -> Self {
        let ZenMesh { vertices, indices } = self;
        let (mesh, _) = indices.iter().fold(
            (
                ZenMesh {
                    vertices: Vec::new(),
                    indices: Vec::new(),
                },
                HashMap::new(),
            ),
            |(mut mesh, mut map), i| {
                let index = if map.contains_key(i) {
                    *map.get(i).unwrap()
                } else {
                    let idx = *i as usize;

                    let vertex = vertices[idx].clone();
                    mesh.vertices.push(vertex);

                    let len = map.len() as u32;
                    map.insert(*i, len);
                    len as u32
                };
                mesh.indices.push(index);
                (mesh, map)
            },
        );

        mesh
    }
}
