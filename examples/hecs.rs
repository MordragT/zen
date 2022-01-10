use hecs::{PreparedQuery, RefMut, World};
use winit::event::*;
use zen_app::App;
use zen_camera::{Camera, FirstPersonCameraBundle, FirstPersonController, Projection};
use zen_core::{EventQueue, Resource, TimeDelta};
use zen_input::KeyboardInput;
use zen_material::Material;
use zen_math::Vec3;
use zen_model::{Mesh, Model};
use zen_texture::Texture;
use zen_window::Window;

pub fn main() {
    println!("Starting Application initialisation...");
    let mut world = World::new();
    let ground = world.spawn((
        Model {
            name: "ground".to_owned(),
            materials: vec![Material::red()],
            meshes: vec![Mesh::plane(10.0)],
        },
        "ground",
    ));
    let mut window = Window::new(world);
    let (width, height) = window.size();
    let bundle = FirstPersonCameraBundle::new(width, height);
    let camera = window.world.spawn((
        bundle.camera,
        bundle.projection,
        bundle.time,
        bundle.controller,
        bundle.keyboard_input,
        bundle.mouse_motion,
    ));
    window.run(move |world| {
        on_input(world);
        on_update(world);
    });
}

pub fn on_input(world: &mut World) {
    zen_camera::on_key(world);
    zen_camera::on_motion(world);
    zen_camera::on_scroll(world);
}

pub fn on_update(world: &mut World) {
    zen_camera::update_camera(world);
}
