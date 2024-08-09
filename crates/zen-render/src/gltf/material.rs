use bevy::asset::LoadContext;
use gltf_json as json;

use crate::{material::ZMat, texture::ZTex};

use super::{error::GltfResult, GltfBuilder};

impl GltfBuilder {
    pub async fn insert_zmat(
        &mut self,
        zmat: &ZMat,
        load_context: &mut LoadContext<'_>,
    ) -> GltfResult<json::Index<json::Material>> {
        let bytes = load_context
            .read_asset_bytes(zmat.texture_asset_path())
            .await?;
        let mut ztex = ZTex::from_bytes(bytes)?;

        let texture = self.insert_ztex(&mut ztex)?;

        let material = self.root.push(json::Material {
            alpha_cutoff: Some(json::material::AlphaCutoff(0.0)),
            alpha_mode: json::validation::Checked::Valid(json::material::AlphaMode::Mask),
            pbr_metallic_roughness: json::material::PbrMetallicRoughness {
                base_color_texture: Some(json::texture::Info {
                    index: texture,
                    tex_coord: 0,
                    extensions: None,
                    extras: Default::default(),
                }),
                metallic_factor: json::material::StrengthFactor(zmat.metallic()),
                ..Default::default()
            },
            ..Default::default()
        });

        Ok(material)
    }
}
