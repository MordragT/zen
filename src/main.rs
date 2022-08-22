use std::fmt::format;

use bevy::{
    prelude::{App, AssetServer, Assets, Commands, Res, ResMut},
    DefaultPlugins,
};
use zen_core::{
    archive::VdfsKind,
    assets::{ZenAssetLoader, ZenAssetLoaderBuilder},
    material::ZenMaterial,
    model::{ZenMesh, ZenModel},
    texture::ZenTexture,
};

pub const GAME_PATH: &'static str = "/home/tom/Steam/steamapps/common/Gothic II";

fn main() -> miette::Result<()> {
    let loader = ZenAssetLoaderBuilder::new()
        .archive(VdfsKind::Mesh, &format!("{GAME_PATH}/Data/Meshes.vdf"))?
        .archive(VdfsKind::Texture, &format!("{GAME_PATH}/Data/Textures.vdf"))?
        .build()?;

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(loader)
        .add_startup_system(load)
        .run();

    Ok(())
}

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     asset_server.add_loader(VdfsLoader);
// }

// fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
//     asset_server.load(&format("{GAME_PATH}/Data/Meshes.vdf#MOSTORCTHRONE02.MRM"))
// }

fn load(
    mut commands: Commands,
    mut zen_loader: ResMut<ZenAssetLoader>,
    model_assets: ResMut<Assets<ZenModel>>,
    mesh_assets: ResMut<Assets<ZenMesh>>,
    material_assets: ResMut<Assets<ZenMaterial>>,
    texture_assets: ResMut<Assets<ZenTexture>>,
) {
    let orc_throne = zen_loader
        .load_model(
            "ORC_MASTERTHRONE.MRM",
            model_assets,
            mesh_assets,
            material_assets,
            texture_assets,
        )
        .expect("Expect to be loaded");

    println!("Throne loaded: {orc_throne:?}");
}
