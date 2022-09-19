use std::fmt::format;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_asset::RenderAssetPlugin, render_resource::WgpuFeatures, settings::WgpuSettings,
    },
    DefaultPlugins,
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use zen_core::{
    archive::VdfsKind,
    assets::ZenAssetLoader,
    material::ZenMaterial,
    model::{ZenMesh, ZenMeshBundle, ZenModel, ZenModelBundle},
    texture::ZenTexture,
};

pub const GAME_PATH: &'static str = "/home/tom/Steam/common/Gothic II";

fn main() -> miette::Result<()> {
    let loader = ZenAssetLoader::new()
        .archive(VdfsKind::Mesh, &format!("{GAME_PATH}/Data/Meshes.vdf"))?
        .archive(VdfsKind::Texture, &format!("{GAME_PATH}/Data/Textures.vdf"))?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(RenderAssetPlugin::<ZenMesh>::default())
        .add_plugin(MaterialPlugin::<ZenMaterial>::default())
        .add_asset::<ZenModel>()
        .add_asset::<ZenMesh>()
        .add_asset::<ZenMaterial>()
        .add_asset::<ZenTexture>()
        .insert_non_send_resource(loader) // TODO fix: make vdfs send
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_startup_system(setup)
        .add_startup_system(load)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = true;

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCam);
}

fn load(
    mut commands: Commands,
    mut zen_loader: NonSend<ZenAssetLoader>,
    mut models: ResMut<Assets<ZenModel>>,
    mut meshes: ResMut<Assets<ZenMesh>>,
    mut materials: ResMut<Assets<ZenMaterial>>,
    mut textures: ResMut<Assets<Image>>,
    mut meshas: ResMut<Assets<Mesh>>,
) {
    log::info!("Starting to load Assets...");

    let model = zen_loader
        .load_model(
            "ORC_MASTERTHRONE.MRM",
            models.as_mut(),
            meshes.as_mut(),
            materials.as_mut(),
            textures.as_mut(),
        )
        .expect("Expect to be loaded");

    // let model = model_assets.as_mut().get(&model).unwrap();
    // commands.spawn_bundle(ZenMeshBundle {
    //     mesh: model.meshes[0].clone(),
    //     ..default()
    // });

    commands
        .spawn_bundle(ZenModelBundle { model, ..default() })
        .insert(meshas.as_mut().add(Mesh::from(shape::Cube { size: 0.2 })));

    commands.spawn_bundle(ZenMeshBundle {
        mesh: meshes.as_mut().add(ZenMesh::plane(50.0)),
        ..default()
    });

    log::info!("Finished loading assets!");
}
