use std::ops::Range;

use wgpu::{util::DeviceExt, RenderPass};
use zen_mesh::Mesh;

pub struct RenderMesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num: u32,
    material: usize,
}

impl RenderMesh {
    pub fn new(device: &wgpu::Device, mesh: &Mesh) -> Self {
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(mesh.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num: mesh.num_elements,
            material: mesh.material,
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>, instances: Range<u32>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.num, 0, instances);
    }

    pub fn num(&self) -> u32 {
        self.num
    }
    pub fn material(&self) -> usize {
        self.material
    }
}
