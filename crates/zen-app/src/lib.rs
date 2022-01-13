use hecs::{PreparedQuery, World};
use winit::{
    error::ExternalError,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use zen_core::{EventQueue, Resource, TimeDelta};
use zen_input::{CursorEntered, CursorLeft, KeyboardInput, MouseInput, MouseMotion, MouseWheel};
use zen_render::{Renderer, WgpuRenderer};

pub struct App<R: Renderer + 'static> {
    event_loop: EventLoop<()>,
    window: Window,
    renderer: R,
    pub world: World,
}

impl App<WgpuRenderer> {
    pub fn new(mut world: World) -> Self {
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = pollster::block_on(WgpuRenderer::new(&window, &mut world));

        Self {
            event_loop,
            window,
            renderer,
            world,
        }
    }
}

impl<R: Renderer + 'static> App<R> {
    pub fn size(&self) -> (u32, u32) {
        self.renderer.size()
    }

    pub fn run(mut self, app: impl Fn(&mut World, &mut Window) + 'static) {
        let mut last_render_time = std::time::Instant::now();

        self.event_loop
            .run(move |event, _, control_flow: &mut ControlFlow| {
                match event {
                    Event::DeviceEvent { ref event, .. } => match event {
                        DeviceEvent::MouseMotion { delta } => {
                            for (_id, queue) in
                                self.world.query_mut::<&mut EventQueue<MouseMotion>>()
                            {
                                queue.push(MouseMotion::new(*delta));
                            }
                        }
                        _ => {}
                    },
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == self.window.id() => match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input:
                                winit::event::KeyboardInput {
                                    state,
                                    virtual_keycode: Some(code),
                                    ..
                                },
                            ..
                        } => {
                            for (_id, queue) in
                                self.world.query_mut::<&mut EventQueue<KeyboardInput>>()
                            {
                                queue.push(KeyboardInput::new(*state, *code));
                            }
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            for (_id, queue) in
                                self.world.query_mut::<&mut EventQueue<MouseWheel>>()
                            {
                                queue.push(MouseWheel::new(*delta));
                            }
                        }
                        WindowEvent::MouseInput {
                            button, // Left Mouse Button
                            state,
                            ..
                        } => {
                            for (_id, queue) in
                                self.world.query_mut::<&mut EventQueue<MouseInput>>()
                            {
                                queue.push(MouseInput::new(*button, *state));
                            }
                        }
                        WindowEvent::CursorEntered { .. } => {
                            for (_id, queue) in
                                self.world.query_mut::<&mut EventQueue<CursorEntered>>()
                            {
                                queue.push(CursorEntered {});
                            }
                        }
                        WindowEvent::CursorLeft { .. } => {
                            for (_id, queue) in
                                self.world.query_mut::<&mut EventQueue<CursorLeft>>()
                            {
                                queue.push(CursorLeft {});
                            }
                        }
                        WindowEvent::Resized(physical_size) => {
                            // for (_id, queue) in self.world.query_mut::<&mut EventQueue<Resized>>() {
                            //     queue.push(Resized::new(physical_size.width, physical_size.height));
                            // }
                            self.renderer.resize(
                                &mut self.world,
                                physical_size.width,
                                physical_size.height,
                            );
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.renderer.resize(
                                &mut self.world,
                                new_inner_size.width,
                                new_inner_size.height,
                            );
                        }
                        _ => {}
                    },

                    Event::RedrawRequested(_) => {
                        let now = std::time::Instant::now();
                        let dt = now - last_render_time;
                        last_render_time = now;
                        self.renderer.update(&mut self.world);
                        match self.renderer.render(&mut self.world) {
                            Ok(_) => {
                                for (_id, delta) in
                                    self.world.query_mut::<&mut Resource<TimeDelta>>()
                                {
                                    delta.replace(dt);
                                }
                            }
                            // // Recreate the swap_chain if lost
                            // Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            // // The system is out of memory, we should probably quit
                            // Err(wgpu::SurfaceError::OutOfMemory) => {
                            //     *control_flow = ControlFlow::Exit
                            // }
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }
                    Event::MainEventsCleared => {
                        // RedrawRequested will only trigger once, unless we manually
                        // request it.
                        self.window.request_redraw();
                    }
                    _ => {}
                };
                app(&mut self.world, &mut self.window);
            });
    }
}
