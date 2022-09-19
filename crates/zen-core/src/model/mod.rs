//! This crate holds data structures required for [Model] objects
//! and different operations on them.

use crate::{material::ZenMaterial, math::Vec3};
use bevy::{
    ecs::system::SystemParamItem,
    prelude::Handle,
    prelude::{Bundle, Component, ComputedVisibility, GlobalTransform, Res, Transform, Visibility},
    reflect::{TypeUuid, Uuid},
    render::{
        extract_component::ExtractComponent,
        mesh::{
            GpuBufferInfo, GpuMesh, InnerMeshVertexBufferLayout, Mesh as BevyMesh,
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
use std::collections::HashMap;
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

#[derive(Bundle, Clone, TypeUuid, Default)]
#[uuid = "2e393245-9977-43a8-97f2-2a0d54700b9d"]
pub struct ZenModelBundle {
    pub model: Handle<ZenModel>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

/// General Mesh consisting out of one or more sub meshes
#[derive(Clone, Debug, Component, TypeUuid)]
#[uuid = "bf78b0e2-3835-11ed-a261-0242ac120002"]
pub struct ZenModel {
    pub meshes: Vec<Handle<ZenMesh>>,
}

// impl ExtractComponent for ZenModel {
//     type Query = &'static Self;
//     type Filter = ();

//     fn extract_component(item: bevy::ecs::query::QueryItem<Self::Query>) -> Self {
//         item.clone()
//     }
// }

#[derive(Bundle, Clone, TypeUuid, Default)]
#[uuid = "4bb17946-383a-11ed-a261-0242ac120002"]
pub struct ZenMeshBundle {
    pub mesh: Handle<ZenMesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

/// Basic Mesh Informations
#[derive(Clone, Debug, Component, TypeUuid)]
#[uuid = "88834d9b-44d4-4686-a570-5cfdd66052b5"]
pub struct ZenMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material: Handle<ZenMaterial>,
    pub num_elements: u32,
}

// TODO impl Into BevyMesh instead of RenderAsset

impl RenderAsset for ZenMesh {
    type ExtractedAsset = ZenMesh;
    type PreparedAsset = GpuMesh;
    type Param = Res<'static, RenderDevice>;

    fn extract_asset(&self) -> Self::ExtractedAsset {
        log::debug!("Extracting ZenMesh for GPU");
        self.clone()
    }

    fn prepare_asset(
        mesh: Self::ExtractedAsset,
        render_device: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        log::debug!("Preparing ZenMesh for GPU");
        let vertex_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            usage: BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&mesh.vertices),
            label: Some("Zen Mesh Vertex Buffer"),
        });

        let index_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            usage: BufferUsages::INDEX,
            contents: bytemuck::cast_slice(&mesh.indices),
            label: Some("Zen Mesh Index Buffer"),
        });

        let buffer_info = GpuBufferInfo::Indexed {
            buffer: index_buffer,
            count: mesh.num_elements,
            index_format: IndexFormat::Uint32,
        };

        let primitive_topology = PrimitiveTopology::TriangleList;
        let mut bevy_mesh = BevyMesh::new(primitive_topology);

        let len = mesh.vertices.len();
        let mut positions = Vec::with_capacity(len);
        let mut normals = Vec::with_capacity(len);
        let mut uvs = Vec::with_capacity(len);

        for vertex in mesh.vertices {
            positions.push(vertex.position);
            normals.push(vertex.normal);
            uvs.push(vertex.tex_coords);
        }

        bevy_mesh.insert_attribute(BevyMesh::ATTRIBUTE_POSITION, positions);
        bevy_mesh.insert_attribute(BevyMesh::ATTRIBUTE_NORMAL, normals);
        bevy_mesh.insert_attribute(BevyMesh::ATTRIBUTE_UV_0, uvs);

        let layout = bevy_mesh.get_mesh_vertex_buffer_layout();

        Ok(GpuMesh {
            vertex_buffer,
            buffer_info,
            primitive_topology,
            layout,
        })
    }
}

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
        Self {
            vertices,
            indices,
            material: Handle::default(),
            num_elements: 6,
        }
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
        let ZenMesh {
            vertices,
            indices,
            material,
            num_elements,
        } = self;
        let (mesh, _) = indices.iter().fold(
            (
                ZenMesh {
                    vertices: Vec::new(),
                    indices: Vec::new(),
                    material,
                    num_elements,
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
