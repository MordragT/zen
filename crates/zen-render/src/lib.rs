use camera::{Camera, CameraController, Projection};
use model::RenderModel;
use wgpu::util::DeviceExt;
use winit::window::Window;

use zen_model::{Mesh, Model, Vertex};

use crate::uniforms::Uniforms;

pub mod camera;
pub mod instance;
pub mod material;
pub mod mesh;
pub mod model;
pub mod texture;
pub mod uniforms;

// pub fn run(model: &Model) {
//     env_logger::init();
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new().build(&event_loop).unwrap();
//     // Since main can't be async, we're going to need to block
//     let mut state = pollster::block_on(State::new(&window, model));
//     let mut last_render_time = std::time::Instant::now();

//     event_loop.run(move |event, _, control_flow| match event {
//         Event::DeviceEvent {
//             ref event,
//             .. // We're not using device_id currently
//         } => {
//             state.input(event);
//         }
//         Event::WindowEvent {
//             ref event,
//             window_id,
//         } if window_id == window.id() => {
//             match event {
//                 WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
//                 WindowEvent::KeyboardInput { input, .. } => match input {
//                     KeyboardInput {
//                         state: ElementState::Pressed,
//                         virtual_keycode: Some(VirtualKeyCode::Escape),
//                         ..
//                     } => {
//                         *control_flow = ControlFlow::Exit;
//                     }
//                     _ => {}
//                 },
//                 WindowEvent::Resized(physical_size) => {
//                     state.resize(*physical_size);
//                 }
//                 WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                     state.resize(**new_inner_size);
//                 }
//                 _ => {}
//             }
//         }
//         Event::RedrawRequested(_) => {
//             let now = std::time::Instant::now();
//             let dt = now - last_render_time;
//             last_render_time = now;
//             state.update(dt);
//             match state.render() {
//                 Ok(_) => {}
//                 // Recreate the swap_chain if lost
//                 Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
//                 // The system is out of memory, we should probably quit
//                 Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
//                 // All other errors (Outdated, Timeout) should be resolved by the next frame
//                 Err(e) => eprintln!("{:?}", e),
//             }
//         }
//         Event::MainEventsCleared => {
//             // RedrawRequested will only trigger once, unless we manually
//             // request it.
//             window.request_redraw();
//         }
//         _ => {}
//     });
// }

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    //sc_desc: wgpu::SwapChainDescriptor,
    //swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    model_state: RenderModel,
    camera: Camera,
    projection: Projection,
    camera_controller: CameraController,
    mouse_pressed: bool,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window, model: &Model) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
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
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let camera = camera::Camera::new((-5.0, 5.0, -1.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let projection =
            camera::Projection::new(size.width, size.height, cgmath::Deg(45.0), 0.1, 100.0);
        let camera_controller = camera::CameraController::new(4.0, 0.4);

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera, &projection);

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

        let texture_bind_group_layout =
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
                        ty: wgpu::BindingType::Sampler {
                            // This is only for TextureSampleType::Depth
                            comparison: false,
                            // This should be true if the sample_type of the texture is:
                            //     TextureSampleType::Float { filterable: true }
                            // Otherwise you'll get an error.
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let model_state = RenderModel::new(&device, &queue, model, &texture_bind_group_layout);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            //flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

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
                entry_point: "main", // 1.
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
                entry_point: "main",
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
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },

            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
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

        Self {
            surface,
            device,
            queue,
            //sc_desc,
            //swap_chain,
            size,
            render_pipeline,
            camera,
            projection,
            camera_controller,
            mouse_pressed: false,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            model_state,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.projection.resize(new_size.width, new_size.height);
        self.size = new_size;
        //self.sc_desc.width = new_size.width;
        //self.sc_desc.height = new_size.height;
        //self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    // fn input(&mut self, event: &DeviceEvent) -> bool {
    //     match event {
    //         DeviceEvent::Key(KeyboardInput {
    //             virtual_keycode: Some(key),
    //             state,
    //             ..
    //         }) => self.camera_controller.process_keyboard(*key, *state),
    //         DeviceEvent::MouseWheel { delta, .. } => {
    //             self.camera_controller.process_scroll(delta);
    //             true
    //         }
    //         DeviceEvent::Button {
    //             button: 1, // Left Mouse Button
    //             state,
    //         } => {
    //             self.mouse_pressed = *state == ElementState::Pressed;
    //             true
    //         }
    //         DeviceEvent::MouseMotion { delta } => {
    //             if self.mouse_pressed {
    //                 self.camera_controller.process_mouse(delta.0, delta.1);
    //             }
    //             true
    //         }
    //         _ => false,
    //     }
    // }

    fn update(&mut self, dt: std::time::Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.uniforms
            .update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_frame()?.output;
        let view = frame
            .texture
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
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            self.model_state.render(&mut render_pass);
            // 2.
            // for (n, (vertex_buffer, index_buffer)) in self
            //     .vertices_buffer
            //     .iter()
            //     .zip(self.indices_buffer.iter())
            //     .enumerate()
            // {
            //     render_pass.set_vertex_buffer(n as u32, vertex_buffer.slice(..));
            //     render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            // }
            // render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 3.        // submit will accept anything that implements IntoIter
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
