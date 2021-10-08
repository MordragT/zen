// use hecs::World;
// use std::time::Duration;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};

// pub trait MouseInput {
//     fn on_button(
//         &mut self,
//         button: MouseButton,
//         state: ElementState,
//         delta: Duration,
//         world: &mut World,
//     );
//     fn on_motion(&mut self, dx: f64, dy: f64, world: &mut World);
//     fn on_scroll(&mut self, delta: MouseScrollDelta, world: &mut World);
//     fn on_cursor_entered(&mut self, world: &mut World);
//     fn on_cursor_left(&mut self, world: &mut World);
// }

// pub trait MouseInput {
//     fn on_button(world: &mut World);
//     fn on_motion(world: &mut World);
//     fn on_scroll(world: &mut World);
//     fn on_cursor_entered(world: &mut World);
//     fn on_cursor_left(world: &mut World);
// }

pub struct MouseMotion {
    pub delta: (f64, f64),
}

impl MouseMotion {
    pub const fn new(delta: (f64, f64)) -> Self {
        Self { delta }
    }
}

pub struct MouseWheel {
    pub delta: MouseScrollDelta,
}

impl MouseWheel {
    pub const fn new(delta: MouseScrollDelta) -> Self {
        Self { delta }
    }
}

pub struct MouseInput {
    pub button: MouseButton,
    pub state: ElementState,
}

impl MouseInput {
    pub const fn new(button: MouseButton, state: ElementState) -> Self {
        Self { button, state }
    }
}
