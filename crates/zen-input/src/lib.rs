pub use keyboard::*;
pub use mouse::*;

mod keyboard;
mod mouse;

//pub trait Input: KeyboardInput + MouseInput {}

pub struct CursorEntered {}
pub struct CursorLeft {}
