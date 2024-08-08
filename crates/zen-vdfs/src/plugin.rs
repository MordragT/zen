use std::{fs::File, io::BufReader};

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
            let reader = BufReader::new(file);
            Box::new(VdfsArchive::from_reader(reader).unwrap())
        });

        app.register_asset_source(AssetSourceId::default(), source);
    }
}
