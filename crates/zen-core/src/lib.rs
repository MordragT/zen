#![feature(half_open_range_patterns)]
#![feature(mixed_integer_ops)]
#![feature(let_chains)]
#![feature(slice_flatten)]
#![feature(variant_count)]

use std::{
    fmt,
    sync::atomic::{AtomicU8, Ordering},
};

use bevy::prelude::{AddAsset, App, MaterialPlugin, Plugin};
use material::ZenMaterial;
use model::{ZenMesh, ZenModel};
use texture::ZenTexture;
use thiserror::Error;

pub mod archive;
//pub mod context;
pub mod assets;
pub mod material;
pub mod math;
pub mod model;
pub mod mrm;
pub mod msh;
pub mod scene;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GameVersion {
    Gothic1,
    Gothic2,
}

impl_try_from_repr!(u8, GameVersion);

static VERSION: AtomicU8 = AtomicU8::new(GameVersion::Gothic2 as u8);

pub fn get_version() -> GameVersion {
    GameVersion::try_from(VERSION.into_inner()).unwrap()
}

pub fn set_version(version: GameVersion) {
    VERSION.swap(version as u8, Ordering::Relaxed);
}

#[derive(Error, Debug)]
pub struct EnumConversionError {
    type_name: &'static str,
    num_type: &'static str,
}

impl fmt::Display for EnumConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error while trying to convert {} into {}",
            self.num_type, self.type_name
        )
    }
}

#[macro_export]
macro_rules! impl_try_from_repr {
    ($repr:ty, $type:ty) => {
        impl TryFrom<$repr> for $type {
            type Error = crate::EnumConversionError;

            fn try_from(value: $repr) -> Result<Self, Self::Error> {
                let count = std::mem::variant_count::<Self>();

                match value {
                    x if x >= 0 && x <= count as $repr => Ok(unsafe { std::mem::transmute(x) }),
                    _ => Err(crate::EnumConversionError {
                        type_name: std::any::type_name::<$type>(),
                        num_type: std::any::type_name::<$repr>(),
                    }),
                }
            }
        }
    };
}
