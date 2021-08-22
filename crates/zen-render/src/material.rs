use crate::texture::RenderTexture;
use zen_material::Material;

pub struct RenderMaterial {
    pub texture: RenderTexture,
}

impl RenderMaterial {
    pub fn create_material(
        material: &Material,
        texture_layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let texture =
            RenderTexture::create_texture(&material.texture, texture_layout, device, queue);
        Self { texture }
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(1, &self.texture.bind_group, &[]);
    }
}
