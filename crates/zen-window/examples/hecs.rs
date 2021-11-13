use hecs::{PreparedQuery, World, RefMut};
use winit::event::*;
use zen_app::App;
use zen_camera::{FirstPersonController, Camera, Projection};
use zen_input::KeyboardInput;
use zen_window::Window;
use zen_core::EventQueue;

pub fn main() {
    println!("Starting Application initialisation...");
    // let app = ZenApplication {};
    let input = ZenInput {
        camera_controller: FirstPersonController::new(4.0, 0.4),
    };
    let mut world = World::new();
    let input_entity = world.spawn((input, EventQueue::<KeyboardInput>::new()));
    let camera_controller = world.spawn(());
    let window = Window::new(world);
    window.run(move |world| {
        let (input, queue) = world.query_one_mut::<(&mut ZenInput, &mut EventQueue<KeyboardInput>)>(input_entity).unwrap();
        input.on_key(queue);
        // for (_id, queue) in world.query_mut::<&mut EventQueue<KeyboardInput>>() {
             
        //     input.on_key(queue);
        // }
    });
}

// struct ZenApplication {}

// impl App for ZenApplication {
//     fn on_init(&mut self, _world: &mut World) {
//         println!("Starting Application...");
//     }
//     fn on_first(&mut self, _world: &mut World) {}
//     fn on_pre_update(&mut self, _world: &mut World) {}
//     fn on_update(&mut self, world: &mut World) {
//         for (_id, queue) in world.query::<&mut EventQueue<KeyboardInput>> {
            
//         }
//     }
//     fn on_post_update(&mut self, _world: &mut World) {}
//     fn on_last(&mut self, _world: &mut World) {}
// }

struct ZenInput {
    camera_controller: FirstPersonController,
}

impl ZenInput {
    fn on_key(&mut self, input: &mut EventQueue<KeyboardInput>) {
        while let Some(KeyboardInput { state, code }) = input.pop() {
            println!("Key: {:?}: {:?}", code, state);
        }
    }
    
    // fn on_motion(&mut self, dx: f64, dy: f64, world: &mut World) {
    //     self.camera_controller.on_motion(dx, dy, world);
    // }
    // fn on_scroll(&mut self, delta: MouseScrollDelta, world: &mut World) {
    //     println!("Scroll wheel delta: {:?}", delta);
    //     self.camera_controller.on_scroll(delta, world);
    // }
    // fn on_cursor_entered(&mut self, world: &mut World) {
    //     println!("Mouse entered");
    //     self.camera_controller.on_cursor_entered(world);
    // }
    // fn on_cursor_left(&mut self, world: &mut World) {
    //     println!("Mouse left");
    //     self.camera_controller.on_cursor_left(world);
    // }

    // fn on_button(&mut self, key: VirtualKeyCode, state: ElementState, world: &mut World) {
    //     // println!("Key: {:?}: {:?}", key, state);
    //     // self.camera_controller.on_key(key, state, world);
    //     match button {
    //         MouseButton::Left => println!("Mouse Left: {:?}", state),
    //         MouseButton::Right => println!("Mouse Right: {:?}", state),
    //         MouseButton::Middle => println!("Mouse Middle: {:?}", state),
    //         MouseButton::Other(b) => println!("Unknown: {}: {:?}", b, state),
    //     }
    //     self.camera_controller.on_button(button, state, world);
    // }
}
