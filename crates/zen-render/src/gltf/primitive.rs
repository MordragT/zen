use crate::math::Vec3;

use gltf_json::{
    self as json,
    validation::{Checked, USize64},
};

use super::GltfBuilder;

/// A simple Vertex
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GltfVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GltfPrimitive {
    pub vertices: Vec<GltfVertex>,
    pub indices: Vec<u32>,
    pub material: Option<json::Index<json::Material>>,
}

impl GltfPrimitive {
    pub fn plane(size: f32) -> Self {
        let vertices = vec![
            GltfVertex {
                position: [0.0, 0.0, 0.0],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, 0.0],
            },
            GltfVertex {
                position: [size, 0.0, 0.0],
                tex_coords: [1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            GltfVertex {
                position: [0.0, 0.0, size],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            GltfVertex {
                position: [size, 0.0, size],
                tex_coords: [1.0, 1.0],
                normal: [1.0, 0.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 1, 3, 2];
        Self {
            vertices,
            indices,
            material: None,
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
        for vertex in self.vertices.iter_mut() {
            vertex.position[0] *= factor;
            vertex.position[1] *= factor;
            vertex.position[2] *= factor;
        }
    }
}

impl GltfBuilder {
    pub fn create_primitive(&mut self, primitive: GltfPrimitive) -> json::mesh::Primitive {
        let (min, max) = primitive.extreme_coordinates();

        let GltfPrimitive {
            vertices,
            indices,
            material,
        } = primitive;

        let vertices_count = USize64::from(vertices.len());
        let vertices_length = self.push_buffer(vertices);

        let vertices_buffer = self.root.push(json::Buffer {
            byte_length: vertices_length,
            name: None,
            uri: None,
            extensions: None,
            extras: None,
        });

        let vertices_view = self.root.push(json::buffer::View {
            buffer: vertices_buffer,
            byte_length: vertices_length,
            byte_offset: None,
            byte_stride: Some(json::buffer::Stride(std::mem::size_of::<GltfVertex>())),
            name: None,
            target: Some(Checked::Valid(json::buffer::Target::ArrayBuffer)),
            extensions: None,
            extras: None,
        });

        let indices_count = USize64::from(indices.len());
        let indices_length = self.push_buffer(indices);

        let indices_buffer = self.root.push(json::Buffer {
            byte_length: indices_length,
            name: None,
            uri: None,
            extensions: None,
            extras: None,
        });

        let indices_view = self.root.push(json::buffer::View {
            buffer: indices_buffer,
            byte_length: indices_length,
            byte_offset: None,
            byte_stride: None,
            name: None,
            target: Some(Checked::Valid(json::buffer::Target::ElementArrayBuffer)),
            extensions: None,
            extras: None,
        });

        let positions = self.root.push(json::Accessor {
            buffer_view: Some(vertices_view),
            byte_offset: Some(USize64(0)),
            count: vertices_count,
            component_type: Checked::Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            type_: Checked::Valid(json::accessor::Type::Vec3),
            min: Some(json::Value::from(min.to_vec())),
            max: Some(json::Value::from(max.to_vec())),
            name: None,
            normalized: false,
            sparse: None,
            extensions: None,
            extras: None,
        });

        let normals = self.root.push(json::Accessor {
            buffer_view: Some(vertices_view),
            byte_offset: Some(USize64((3 * std::mem::size_of::<f32>()) as u64)),
            count: vertices_count,
            component_type: Checked::Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            type_: Checked::Valid(json::accessor::Type::Vec3),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
            extensions: None,
            extras: None,
        });

        let tex_coords = self.root.push(json::Accessor {
            buffer_view: Some(vertices_view),
            byte_offset: Some(USize64((6 * std::mem::size_of::<f32>()) as u64)),
            count: vertices_count,
            component_type: Checked::Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::F32,
            )),
            type_: Checked::Valid(json::accessor::Type::Vec2),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
            extensions: None,
            extras: None,
        });

        let indices = self.root.push(json::Accessor {
            buffer_view: Some(indices_view),
            byte_offset: Some(USize64(0)),
            count: indices_count,
            component_type: Checked::Valid(json::accessor::GenericComponentType(
                json::accessor::ComponentType::U32,
            )),
            type_: Checked::Valid(json::accessor::Type::Scalar),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
            extensions: None,
            extras: None,
        });

        let primitive = json::mesh::Primitive {
            attributes: {
                let mut map = std::collections::BTreeMap::new();
                map.insert(Checked::Valid(json::mesh::Semantic::Positions), positions);
                map.insert(Checked::Valid(json::mesh::Semantic::Normals), normals);
                map.insert(
                    Checked::Valid(json::mesh::Semantic::TexCoords(0)),
                    tex_coords,
                );
                map
            },
            indices: Some(indices),
            material,
            mode: Checked::Valid(json::mesh::Mode::Triangles),
            targets: None,
            extensions: None,
            extras: None,
        };

        primitive
    }
}
