use std::fmt::format;

use bevy::{
    prelude::{App, AssetServer, Commands, Res},
    DefaultPlugins,
};
use zen_core::archive::bevy::VdfsLoader;

pub const GAME_PATH: &'static str = "/opt/SteamLibrary/steamapps/common/Gothic II";

fn main() -> miette::Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.add_loader(VdfsLoader);
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.load(&format("{GAME_PATH}/Data/Meshes.vdf#MOSTORCTHRONE02.MRM"))
}
