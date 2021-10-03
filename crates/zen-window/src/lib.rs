use hecs::World;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};
use zen_app::App;
use zen_input::Input;

pub mod error;

pub struct Window<A: App + 'static, I: Input + 'static> {
    event_loop: EventLoop<()>,
    window: WinitWindow,
    app: A,
    input: I,
    pub world: World,
}

impl<A: App, I: Input> Window<A, I> {
    pub fn new(app: A, input: I, world: World) -> Self {
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        Self {
            event_loop,
            window,
            app,
            input,
            world,
        }
    }

    pub fn run(mut self) {
        self.app.on_init(&mut self.world);
        self.event_loop
            .run(move |event, _, control_flow: &mut ControlFlow| {
                match event {
                    Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == self.window.id() => match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(key_code),
                                    ..
                                },
                            ..
                        } => match key_code {
                            VirtualKeyCode::A => self.input.on_a(&mut self.world),
                            VirtualKeyCode::B => self.input.on_b(&mut self.world),
                            VirtualKeyCode::C => self.input.on_c(&mut self.world),
                            VirtualKeyCode::D => self.input.on_d(&mut self.world),
                            VirtualKeyCode::E => self.input.on_e(&mut self.world),
                            VirtualKeyCode::F => self.input.on_f(&mut self.world),
                            VirtualKeyCode::G => self.input.on_g(&mut self.world),
                            VirtualKeyCode::H => self.input.on_h(&mut self.world),
                            VirtualKeyCode::I => self.input.on_i(&mut self.world),
                            VirtualKeyCode::J => self.input.on_j(&mut self.world),
                            VirtualKeyCode::K => self.input.on_k(&mut self.world),
                            VirtualKeyCode::L => self.input.on_l(&mut self.world),
                            VirtualKeyCode::M => self.input.on_m(&mut self.world),
                            VirtualKeyCode::N => self.input.on_n(&mut self.world),
                            VirtualKeyCode::O => self.input.on_o(&mut self.world),
                            VirtualKeyCode::P => self.input.on_p(&mut self.world),
                            VirtualKeyCode::Q => self.input.on_q(&mut self.world),
                            VirtualKeyCode::R => self.input.on_r(&mut self.world),
                            VirtualKeyCode::S => self.input.on_s(&mut self.world),
                            VirtualKeyCode::T => self.input.on_t(&mut self.world),
                            VirtualKeyCode::U => self.input.on_u(&mut self.world),
                            VirtualKeyCode::V => self.input.on_v(&mut self.world),
                            VirtualKeyCode::W => self.input.on_w(&mut self.world),
                            VirtualKeyCode::X => self.input.on_x(&mut self.world),
                            VirtualKeyCode::Y => self.input.on_y(&mut self.world),
                            VirtualKeyCode::Z => self.input.on_z(&mut self.world),
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                };
                self.app.on_first(&mut self.world);
                self.app.on_pre_update(&mut self.world);
                self.app.on_update(&mut self.world);
                self.app.on_post_update(&mut self.world);
                self.app.on_last(&mut self.world);
            });
    }
}
