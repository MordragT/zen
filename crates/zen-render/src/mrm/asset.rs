use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    gltf::{Gltf, GltfAssetLabel, GltfMesh},
};

use super::{Mrm, MrmError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrmLoader {}

impl AssetLoader for MrmLoader {
    type Asset = Gltf;
    type Settings = ();
    type Error = MrmError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        settings: &'a Self::Settings,
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        // let MrmBundle {
        //     vertices,
        //     normals,
        //     materials,
        //     meshes,
        //     alpha_test,
        //     bounding_box,
        // } = MrmBundle::from_bytes(bytes)?;

        // let materials = materials
        //     .into_iter()
        //     .map(|zmat| {
        //         let name = zmat.name().to_owned();
        //         let mat = load_material(zmat, load_context);

        //         load_context.add_labeled_asset(name, mat)
        //     })
        //     .collect();

        // let meshes = meshes
        //     .into_iter()
        //     .enumerate()
        //     .map(|(index, mesh)| {
        //         let name = format!("GltfMesh{}", index);

        //         let primitives = todo!();

        //         let mesh = GltfMesh {
        //             index,
        //             asset_label: GltfAssetLabel::Mesh(index),
        //             name,
        //             primitives,
        //             extras: None,
        //         };

        //         load_context.add_labeled_asset(name, mesh)
        //     })
        //     .collect();

        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &["MRM", "mrm"]
    }
}
