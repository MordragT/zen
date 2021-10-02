pub use keyboard::KeyboardInput;
pub use mouse::MouseInput;

mod keyboard;
mod mouse;

pub trait Input: KeyboardInput + MouseInput {}
