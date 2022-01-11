use hecs::{PreparedQuery, RefMut, World};
use std::{convert::TryFrom, fs::File, io::Cursor};
use winit::{event::*, window::Window};
use zen_app::{App, EventQueue, Resource, TimeDelta};
use zen_archive::Vdfs;
use zen_first_person_camera::{FirstPersonCameraBundle, FirstPersonController};
use zen_input::KeyboardInput;
use zen_material::Material;
use zen_math::Vec3;
use zen_model::{Mesh, Model};
use zen_mrm::Mrm;
use zen_texture::Texture;
use zen_types::path::INSTANCE;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let vdf_file = File::open(INSTANCE.meshes())?;
    let vdf = Vdfs::new(vdf_file)?;
    let mesh_entry = vdf
        .get_by_name("ORC_MASTERTHRONE.MRM")
        .expect("Should be there!");
    let cursor = Cursor::new(mesh_entry.data);
    let throne_mesh = Mrm::new(cursor, "ORC_MASTERTHRONE")?;
    let throne_model = Model::try_from(throne_mesh)?;
    let throne = world.spawn((throne_model, "throne"));

    let mut window = App::new(world);
    let (width, height) = window.size();
    let bundle = FirstPersonCameraBundle::new(width, height);
    let camera = window.world.spawn((
        bundle.camera,
        bundle.projection,
        bundle.time,
        bundle.controller,
        bundle.keyboard_input,
        bundle.mouse_motion,
        bundle.mouse_input,
    ));
    window.run(move |world, window| {
        on_input(world, window);
        on_update(world);
    });
    Ok(())
}

pub fn on_input(world: &mut World, window: &Window) {
    zen_first_person_camera::on_button(world, window);
    zen_first_person_camera::on_key(world);
    zen_first_person_camera::on_motion(world);
    zen_first_person_camera::on_scroll(world);
}

pub fn on_update(world: &mut World) {
    zen_first_person_camera::update_camera(world);
}
