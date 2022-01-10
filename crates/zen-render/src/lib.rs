use wgpu::util::DeviceExt;
use winit::window::Window;
use zen_camera::{Camera, Projection};
use hecs::{World, PreparedQuery};
use zen_model::{Mesh, Model, Vertex};
use zen_texture::Texture;
use crate::uniforms::Uniforms;
use bundles::{ModelBundle, MeshBundle};
use draw::DrawModel;

pub mod draw;
pub mod uniforms;
mod bundles;

pub trait Renderer {
    type Error: std::error::Error;
    // fn add_mesh(&mut self, mesh: &Mesh) -> (usize, usize);
    // fn add_texture(&mut self) -> usize;    
    fn size(&self) -> (u32, u32);
    fn resize(&mut self, world: &mut World, width: u32, height: u32);
    fn update(&mut self, world: &mut World);
    fn render(&mut self, world: &mut World) -> Result<(), Self::Error>;
}

pub struct WgpuRenderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    //sc_desc: wgpu::SwapChainDescriptor,
    //swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
    config: wgpu::SurfaceConfiguration,
    // vertex_buffers: Vec<wgpu::Buffer>,
    // index_buffers: Vec<wgpu::Buffer>,
    // texture_bind_groups: Vec<wgpu::BindGroup>,
    //model_states: Vec<GpuModel>,
    //camera: Camera,
    //projection: Projection,
    //camera_controller: CameraController,
    mouse_pressed: bool,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}


impl Renderer for WgpuRenderer {
    type Error = wgpu::SurfaceError;
    // fn add_mesh(&mut self, mesh: &Mesh) -> (usize, usize) {
    //     let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //         label: Some("Index Buffer"),
    //         contents: bytemuck::cast_slice(mesh.indices.as_slice()),
    //         usage: wgpu::BufferUsages::INDEX,
    //     });
    //     let index_buffer_id = self.index_buffers.push(index_buffer);

    //     let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //         label: Some("Vertex Buffer"),
    //         contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
    //         usage: wgpu::BufferUsages::VERTEX,
    //     });
    //     let vertex_buffer_id = self.vertex_buffers.push(vertex_buffer);
        
    //     (vertex_buffer, index_buffer)
    // }
    
    // fn add_texture(&mut self, )
    fn size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }
    
    fn resize(&mut self, world: &mut World, width: u32, height: u32) {
        //self.projection.resize(new_size.width, new_size.height);
        for (_id, projection) in world.query_mut::<&mut Projection>() {
            projection.resize(width, height);
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn update(&mut self, world: &mut World) {
        for (_id, (camera, projection)) in world.query_mut::<(&mut Camera, &mut Projection)>() {
        //self.camera_controller.update_camera(&mut self.camera, dt);
        self.uniforms
            .update_view_proj(&camera, &projection);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );}
    }

    fn render(&mut self, world: &mut World) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            // render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            // TODO add DrawModel trait functions            
            for (id, model) in world.query_mut::<&mut ModelBundle>() {
                render_pass.draw_model(model, &self.uniform_bind_group);                
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}

pub fn light_bind_group_layout(device: &mut wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        })
}

pub fn texture_bind_group_layout(device: &mut wgpu::Device) -> wgpu::BindGroupLayout {
    
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        // Uint due to texture::format
                        sample_type: wgpu::TextureSampleType::Uint,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
}

impl WgpuRenderer {    
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window, world: &mut World) -> Self {
        let physical_size = window.inner_size();
        let size = (physical_size.width, physical_size.height);

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (mut device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        // let sc_desc = wgpu::SwapChainDescriptor {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
        //     width: size.width,
        //     height: size.height,
        //     present_mode: wgpu::PresentMode::Fifo,
        // };
        // let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        //let camera = FirstPersonCamera::new((-5.0, 5.0, -1.0), -90.0, -20.0);
        //let projection = Projection::new(size.width, size.height, 45.0, 0.1, 100.0);
        //let camera_controller = camera::CameraController::new(4.0, 0.4);

        let mut uniforms = Uniforms::new();
        // TODO update view projection with hecs world query
        // uniforms.update_view_proj(&camera, &projection);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

            
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            //flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        
        let texture_bind_group_layout = 
texture_bind_group_layout(&mut device);
        
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }], // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Front),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },

            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None,
        });
        

        // let (num_vertices, vertices_buffer, num_indices, indices_buffer) = scene.into_iter().fold(
        //     (0, Vec::new(), 0, Vec::new()),
        //     |(mut num_vertices, mut vertices_buffer, mut num_indices, mut indices_buffer),
        //      general| {
        //         for mesh in general.meshes {
        //             num_vertices += (mesh.positions.len() / 3) as u32;
        //             vertices_buffer.push(device.create_buffer_init(
        //                 &wgpu::util::BufferInitDescriptor {
        //                     label: Some(&general.name),
        //                     contents: bytemuck::cast_slice(mesh.positions.as_slice()),
        //                     usage: wgpu::BufferUsage::VERTEX,
        //                 },
        //             ));
        //             num_indices += mesh.indices.len() as u32;
        //             indices_buffer.push(device.create_buffer_init(
        //                 &wgpu::util::BufferInitDescriptor {
        //                     label: Some(&general.name),
        //                     contents: bytemuck::cast_slice(mesh.indices.as_slice()),
        //                     usage: wgpu::BufferUsage::INDEX,
        //                 },
        //             ))
        //         }
        //         (num_vertices, vertices_buffer, num_indices, indices_buffer)
        //     },
        // );

        //let num_vertices = (mesh.indices.len()) as u32;

        let mut renderer = Self {
            surface,
            device,
            queue,
            //sc_desc,
            //swap_chain,
            config,
            render_pipeline,
            //camera,
            // projection,
            //camera_controller,
            mouse_pressed: false,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            //model_state,
        };
        
        let models = world.query::<&Model>().iter().map(|(_id, model)| bundles::ModelBundle::load(&model, &mut renderer)).collect::<Vec<ModelBundle>>();
        for model in models {
            world.spawn((model, ""));
        }
        
        renderer
    }

}
