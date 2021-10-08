// use hecs::World;
// use std::time::Duration;
use winit::event::{ElementState, VirtualKeyCode};

// pub trait KeyboardInput {
//     fn on_key(
//         &mut self,
//         key: VirtualKeyCode,
//         state: ElementState,
//         delta: Duration,
//         world: &mut World,
//     );
// }

// pub trait KeyboardInput {
//     fn on_key(world: &mut World);
// }

pub struct KeyboardInput {
    pub state: ElementState,
    pub code: VirtualKeyCode,
}

impl KeyboardInput {
    pub const fn new(state: ElementState, code: VirtualKeyCode) -> Self {
        Self { state, code }
    }
}
