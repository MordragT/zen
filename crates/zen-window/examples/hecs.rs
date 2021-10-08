use hecs::{PreparedQuery, World};
use winit::event::*;
use zen_app::App;
use zen_camera::FirstPersonCameraController;
use zen_input::{KeyboardInput, MouseInput};
use zen_window::Window;

pub fn main() {
    let app = ZenApplication {};
    let input = ZenInput {
        camera_controller: FirstPersonCameraController::new(4.0, 0.4),
    };
    let world = World::new();
    let window = Window::new(app, input, world);
    window.run();
}

struct ZenApplication {}

impl App for ZenApplication {
    fn on_init(&mut self, _world: &mut World) {
        println!("Starting Application...");
    }
    fn on_first(&mut self, _world: &mut World) {}
    fn on_pre_update(&mut self, _world: &mut World) {}
    fn on_update(&mut self, _world: &mut World) {}
    fn on_post_update(&mut self, _world: &mut World) {}
    fn on_last(&mut self, _world: &mut World) {}
}

struct ZenInput {
    camera_controller: FirstPersonCameraController,
}

impl Input for ZenInput {}

impl MouseInput for ZenInput {
    fn on_button(&mut self, button: MouseButton, state: ElementState, world: &mut World) {
        match button {
            MouseButton::Left => println!("Mouse Left: {:?}", state),
            MouseButton::Right => println!("Mouse Right: {:?}", state),
            MouseButton::Middle => println!("Mouse Middle: {:?}", state),
            MouseButton::Other(b) => println!("Unknown: {}: {:?}", b, state),
        }
        self.camera_controller.on_button(button, state, world);
    }
    fn on_motion(&mut self, dx: f64, dy: f64, world: &mut World) {
        self.camera_controller.on_motion(dx, dy, world);
    }
    fn on_scroll(&mut self, delta: MouseScrollDelta, world: &mut World) {
        println!("Scroll wheel delta: {:?}", delta);
        self.camera_controller.on_scroll(delta, world);
    }
    fn on_cursor_entered(&mut self, world: &mut World) {
        println!("Mouse entered");
        self.camera_controller.on_cursor_entered(world);
    }
    fn on_cursor_left(&mut self, world: &mut World) {
        println!("Mouse left");
        self.camera_controller.on_cursor_left(world);
    }
}

impl KeyboardInput for ZenInput {
    fn on_key(&mut self, key: VirtualKeyCode, state: ElementState, world: &mut World) {
        println!("Key: {:?}: {:?}", key, state);
        self.camera_controller.on_key(key, state, world);
    }
}
