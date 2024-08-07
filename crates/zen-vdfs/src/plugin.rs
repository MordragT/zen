use std::fs::File;

use bevy::{
    app::{App, Plugin},
    asset::{
        io::{AssetSource, AssetSourceId},
        AssetApp, AssetPlugin,
    },
};

use crate::VdfsArchive;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VdfsPlugin {
    pub path: &'static str,
}

impl Plugin for VdfsPlugin {
    fn build(&self, app: &mut App) {
        if app.is_plugin_added::<AssetPlugin>() {
            bevy::log::error!("VdfsPlugin must be added before AssetPlugin");
        }

        let path = self.path;

        let source = AssetSource::build().with_reader(move || {
            let file = File::open(path).unwrap();
            Box::new(VdfsArchive::new(file).unwrap())
        });

        app.register_asset_source(AssetSourceId::default(), source);
    }
}
