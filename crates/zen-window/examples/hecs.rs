use hecs::{PreparedQuery, World};
use zen_app::App;
use zen_input::{Input, KeyboardInput, MouseInput};
use zen_window::Window;

pub fn main() {
    let app = ZenApplication {};
    let input = ZenInput {};
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

struct ZenInput {}

impl Input for ZenInput {}

impl MouseInput for ZenInput {
    fn on_left_click(&mut self, _world: &mut World) {
        println!("left clicked");
    }
    fn on_right_click(&mut self, _world: &mut World) {
        println!("right clicked");
    }
}

impl KeyboardInput for ZenInput {
    fn on_a(&mut self, _world: &mut World) {
        println!("a pressed");
    }
    fn on_b(&mut self, _world: &mut World) {
        println!("b pressed");
    }
    fn on_c(&mut self, _world: &mut World) {
        println!("c pressed");
    }
    fn on_d(&mut self, _world: &mut World) {
        println!("d pressed");
    }
    fn on_e(&mut self, _world: &mut World) {
        println!("e pressed");
    }
    fn on_f(&mut self, _world: &mut World) {
        println!("f pressed");
    }
    fn on_g(&mut self, _world: &mut World) {
        println!("g pressed");
    }
    fn on_h(&mut self, _world: &mut World) {
        println!("h pressed");
    }
    fn on_i(&mut self, _world: &mut World) {
        println!("i pressed");
    }
    fn on_j(&mut self, _world: &mut World) {
        println!("j pressed");
    }
    fn on_k(&mut self, _world: &mut World) {
        println!("k pressed");
    }
    fn on_l(&mut self, _world: &mut World) {
        println!("l pressed");
    }
    fn on_m(&mut self, _world: &mut World) {
        println!("m pressed");
    }
    fn on_n(&mut self, _world: &mut World) {
        println!("n pressed");
    }
    fn on_o(&mut self, _world: &mut World) {
        println!("o pressed");
    }
    fn on_p(&mut self, _world: &mut World) {
        println!("p pressed");
    }
    fn on_q(&mut self, _world: &mut World) {
        println!("q pressed");
    }
    fn on_r(&mut self, _world: &mut World) {
        println!("r pressed");
    }
    fn on_s(&mut self, _world: &mut World) {
        println!("s pressed");
    }
    fn on_t(&mut self, _world: &mut World) {
        println!("t pressed");
    }
    fn on_u(&mut self, _world: &mut World) {
        println!("u pressed");
    }
    fn on_v(&mut self, _world: &mut World) {
        println!("v pressed");
    }
    fn on_w(&mut self, _world: &mut World) {
        println!("w pressed");
    }
    fn on_x(&mut self, _world: &mut World) {
        println!("x pressed");
    }
    fn on_y(&mut self, _world: &mut World) {
        println!("y pressed");
    }
    fn on_z(&mut self, _world: &mut World) {
        println!("z pressed");
    }
}
