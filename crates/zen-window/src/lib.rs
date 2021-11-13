use hecs::{World, PreparedQuery};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};
use zen_app::App;
use zen_camera::{Projection, Camera};
use zen_core::EventQueue;
use zen_input::{CursorEntered, CursorLeft, KeyboardInput, MouseInput, MouseMotion, MouseWheel};
use zen_render::{Renderer, Resized};

pub mod error;

pub struct Window {
    event_loop: EventLoop<()>,
    window: WinitWindow,
    // app: FnMut(&mut World),
    renderer: Renderer,
    pub world: World,
}

impl Window {
    pub fn new(world: World) -> Self {
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = pollster::block_on(Renderer::new(&window));

        Self {
            event_loop,
            window,
            // app,
            renderer,
            world,
        }
    }

    pub fn run(mut self, app: impl Fn(&mut World) + 'static) {
        let mut camera_projection = PreparedQuery::<(&Camera, &Projection)>::default();
        
        // self.app.on_init(&mut self.world);
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
                        WindowEvent::Resized(physical_size) => for (_id, queue) in self.world.query_mut::<&mut EventQueue<Resized>>() {
                            queue.push(Resized::new(physical_size.width, physical_size.height));
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.renderer.resize(**new_inner_size);
                        }
                        _ => {}
                    },

                    Event::RedrawRequested(_) => {
                        // let now = std::time::Instant::now();
                        // let dt = now - last_render_time;
                        // last_render_time = now;
                        self.renderer.update(&mut self.world, &mut camera_projection);
                        match self.renderer.render() {
                            Ok(_) => {}
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
                app(&mut self.world);
                // self.app.on_first(&mut self.world);
                // self.app.on_pre_update(&mut self.world);
                // self.app.on_update(&mut self.world);
                // self.app.on_post_update(&mut self.world);
                // self.app.on_last(&mut self.world);
                //  self.world.get_mut(mouse_motion) -> mouse_motion = None
            });
    }
}
