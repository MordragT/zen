use bevy::prelude::*;

use super::ZTexLoader;

// use crate::{PngBuffer, ZTexImageLoader, ZTexPngBufferLoader};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZTexPlugin {}

// impl Plugin for ZTexPlugin {
//     fn build(&self, app: &mut App) {
//         app.init_asset::<PngBuffer>()
//             .preregister_asset_loader::<ZTexImageLoade>(&["TEX", "tex"])
//             .preregister_asset_loader::<ZTexPngBufferLoader>(&["TEX", "tex"]);
//     }

//     fn finish(&self, app: &mut App) {
//         app.register_asset_loader(ZTexImageLoader {})
//             .register_asset_loader(ZTexPngBufferLoader {});
//     }
// }

impl Plugin for ZTexPlugin {
    fn build(&self, app: &mut App) {
        app.preregister_asset_loader::<ZTexLoader>(&["TEX", "tex"]);
    }

    fn finish(&self, app: &mut App) {
        app.register_asset_loader(ZTexLoader {});
    }
}
