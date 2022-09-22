use std::fmt::format;

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        render_asset::RenderAssetPlugin, render_resource::WgpuFeatures, settings::WgpuSettings,
        view::NoFrustumCulling,
    },
    DefaultPlugins,
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use zen_core::{
    archive::VdfsKind,
    assets::{ZenAssetLoader, ZenLoadContext},
    material::ZenMaterial,
    model::{gltf::Output, ZenMesh, ZenModel},
    texture::ZenTexture,
    ZenPlugin,
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
        .add_plugin(ZenPlugin)
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
    mut textures: ResMut<Assets<ZenTexture>>,
    mut bevy_meshes: ResMut<Assets<Mesh>>,
    mut bevy_materials: ResMut<Assets<StandardMaterial>>,
    mut bevy_textures: ResMut<Assets<Image>>,
) {
    log::info!("Starting to load Assets...");

    let mut context = ZenLoadContext::new(meshes.as_mut(), materials.as_mut(), textures.as_mut());

    let model = zen_loader
        .load_model("ORC_MASTERTHRONE.MRM", &mut context)
        .unwrap();
    let out_path = model.to_gltf(&mut context, Output::Binary);

    println!("Model exported to {out_path:?}");

    // let entity = zen_loader
    //     .spawn_model(
    //         model,
    //         bevy_meshes.as_mut(),
    //         bevy_materials.as_mut(),
    //         bevy_textures.as_mut(),
    //         &mut context,
    //         &mut commands,
    //     )
    //     .expect("Expect to be loaded");

    // println!("Throne loaded: {model:?}");

    // let model = model_assets.as_mut().get(&model).unwrap();
    // commands.spawn_bundle(ZenMeshBundle {
    //     mesh: model.meshes[0].clone(),
    //     ..default()
    // });

    // commands
    //     .spawn_bundle(ZenModelBundle { model, ..default() })
    //     .insert(NoFrustumCulling)
    //     .insert(
    //         bevy_meshes
    //             .as_mut()
    //             .add(Mesh::from(shape::Cube { size: 0.2 })),
    //     );

    // commands.spawn_bundle(ZenMeshBundle {
    //     mesh: meshes.as_mut().add(ZenMesh::plane(50.0)),
    //     ..default()
    //});

    log::info!("Finished loading assets!");
}
