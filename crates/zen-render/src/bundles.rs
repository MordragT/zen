use crate::Renderer;
use crate::WgpuRenderer;
use hecs::Bundle;
use hecs_macros::Bundle;
use wgpu::util::DeviceExt;
use zen_material::Material;
use zen_model::{Mesh, Model};
use zen_texture::Texture;

pub struct MeshBundle {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl MeshBundle {
    pub fn load(mesh: &Mesh, renderer: &mut WgpuRenderer) -> Self {
        let index_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(mesh.indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });

        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        Self {
            vertex_buffer,
            index_buffer,
            num_elements: mesh.num_elements,
            material: mesh.material,
        }
    }
}

pub struct ModelBundle {
    pub meshes: Vec<MeshBundle>,
    pub materials: Vec<wgpu::BindGroup>,
}

impl ModelBundle {
    pub fn load(model: &Model, renderer: &mut WgpuRenderer) -> Self {
        let meshes = model
            .meshes
            .iter()
            .map(|mesh| MeshBundle::load(mesh, renderer))
            .collect();
        let materials = model
            .materials
            .iter()
            .map(|material| load_texture(&material.texture, renderer))
            .collect();

        Self { meshes, materials }
    }
}

fn load_texture(texture: &Texture, renderer: &mut WgpuRenderer) -> wgpu::BindGroup {
    let diffuse_texture = renderer.device.create_texture(&texture.desc());
    renderer.queue.write_texture(
        // Tells wgpu where to copy the pixel data
        wgpu::ImageCopyTexture {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        // The actual pixel data
        texture.as_bytes(),
        // The layout of the texture
        texture.layout(),
        texture.extend_3d(),
    );

    // We don't need to configure the texture view much, so let's
    // let wgpu define it.
    let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(texture.format()),
        ..Default::default()
    });
    let diffuse_sampler = renderer.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    let bind_group_layout = crate::texture_bind_group_layout(&mut renderer.device);

    let bind_group = renderer
        .device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

    bind_group
}
