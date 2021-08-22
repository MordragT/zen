use zen_mesh::Model;

use crate::{material::RenderMaterial, mesh::RenderMesh, texture::RenderTexture};

pub struct RenderModel {
    pub meshes: Vec<RenderMesh>,
    pub materials: Vec<RenderMaterial>,
}

impl RenderModel {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model: &Model,
        texture_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let meshes = model
            .meshes
            .iter()
            .map(|mesh| RenderMesh::new(device, mesh))
            .collect();
        let materials = model
            .materials
            .iter()
            .map(|material| {
                RenderMaterial::create_material(material, texture_layout, device, queue)
            })
            .collect();
        Self { meshes, materials }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        for state in self.meshes.iter() {
            state.render(render_pass, 0..1);
            self.materials[state.material()].render(render_pass);
        }
    }
}
