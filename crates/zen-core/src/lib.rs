#![feature(half_open_range_patterns)]
#![feature(mixed_integer_ops)]
#![feature(let_chains)]

use bevy::prelude::{AddAsset, App, MaterialPlugin, Plugin};
use material::ZenMaterial;
use model::{ZenMesh, ZenModel};
use texture::ZenTexture;

pub mod archive;
//pub mod context;
pub mod assets;
pub mod material;
pub mod math;
pub mod model;
pub mod mrm;
pub mod msh;
pub mod texture;

#[derive(Default)]
pub struct ZenPlugin;

impl Plugin for ZenPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<ZenModel>()
            .add_asset::<ZenMesh>()
            .add_asset::<ZenMaterial>()
            .add_asset::<ZenTexture>();
        // .add_plugin(MaterialPlugin::<ZenMaterial>::default());
    }
}
