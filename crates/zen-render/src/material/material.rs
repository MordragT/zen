use std::num::ParseIntError;

use bevy::asset::LoadContext;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use zen_core::GameKind;
use zen_parser::binary::{BinaryDecoder, BinaryRead};

use super::header::ZMatHeader;
use super::{ZMatKind, ZMatResult};
use crate::math::{Vec2, Vec4};

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct ZMat {
    header: ZMatHeader,
    name: String,
    kind: ZMatKind,
    color: Vec4<u8>,
    smooth_angle: u32,
    texture: ZMatTexture,
    disable_collision: bool,
    disable_lightmap: bool,
    dont_collapse: bool,
    detail_object: String,
    extra: Option<ZMatExtra>,
    default_mapping: Vec2<f32>,
}

impl ZMat {
    pub fn from_decoder<R>(decoder: &mut BinaryDecoder<R>) -> ZMatResult<Self>
    where
        R: BinaryRead,
    {
        let header = decoder.decode::<ZMatHeader>()?;
        header.validate()?;

        let name = decoder.decode::<String>()?;
        let kind = decoder.decode::<ZMatKind>()?;
        let color = decoder.decode::<Vec4<u8>>()?;
        let smooth_angle = decoder.decode::<u32>()?;
        let texture = ZMatTexture::from_decoder(decoder)?;
        let disable_collision = decoder.decode::<bool>()?;
        let disable_lightmap = decoder.decode::<bool>()?;
        let dont_collapse = decoder.decode::<bool>()?;
        let detail_object = decoder.decode::<String>()?;
        let extra = if header.kind() == GameKind::Gothic2 {
            Some(decoder.decode::<ZMatExtra>()?)
        } else {
            None
        };
        let default_mapping = decoder.decode::<Vec2<f32>>()?;

        Ok(Self {
            header,
            name,
            kind,
            color,
            smooth_angle,
            texture,
            disable_collision,
            disable_lightmap,
            dont_collapse,
            detail_object,
            extra,
            default_mapping,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> ZMatKind {
        self.kind
    }

    pub fn color(&self) -> Vec4<u8> {
        self.color
    }

    pub fn metallic(&self) -> f32 {
        match self.kind {
            ZMatKind::Undef => 0.0,
            ZMatKind::Metal => 1.0,
            ZMatKind::Stone => 0.5,
            ZMatKind::Wood => 0.0,
            ZMatKind::Earth => 0.0,
            ZMatKind::Water => 0.0,
            ZMatKind::Snow => 0.0,
        }
    }

    pub fn texture(&self) -> &String {
        &self.texture.path
    }

    pub fn texture_asset_path(&self) -> String {
        let (name, _end) = self
            .texture
            .path
            .split_once('.')
            .expect("Every texture has an ending");
        format!("texture://{name}-C.TEX")
    }

    pub fn to_standard_material(&self, load_context: &mut LoadContext<'_>) -> StandardMaterial {
        let image_handle: Handle<Image> = load_context.load(self.texture_asset_path());

        let color = LinearRgba::from_u8_array(self.color().to_array());

        // TODO check if values are fine
        let (metallic, perceptual_roughness, reflectance) = match self.kind() {
            ZMatKind::Undef => (0.0, 0.5, 0.5),
            ZMatKind::Metal => (1.0, 0.25, 0.85),
            ZMatKind::Stone => (0.5, 0.5, 0.6),
            ZMatKind::Wood => (0.0, 0.7, 0.5),
            ZMatKind::Earth => (0.0, 0.9, 0.5),
            ZMatKind::Water => (0.0, 0.5, 0.75),
            ZMatKind::Snow => (0.0, 0.8, 0.5),
        };

        StandardMaterial {
            base_color: color.into(),
            metallic,
            perceptual_roughness,
            reflectance,
            base_color_texture: Some(image_handle),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
struct ZMatTexture {
    path: String,
    scale: Vec2<u32>,
    anim_fps: f32,
    linear_anim_mapping: bool,
    anim_mapping_dir: Vec2<u32>,
}

impl ZMatTexture {
    pub fn from_decoder<R>(decoder: &mut BinaryDecoder<R>) -> ZMatResult<Self>
    where
        R: BinaryRead,
    {
        let path = decoder.decode::<String>()?;
        let scale_str = decoder.decode::<String>()?;
        let scale = str_to_vec2(&scale_str)?;
        let anim_fps = decoder.decode::<f32>()?;
        let linear_anim_mapping = decoder.decode::<bool>()?;
        let anim_mapping_dir_str = decoder.decode::<String>()?;
        let anim_mapping_dir = str_to_vec2(&anim_mapping_dir_str)?;

        Ok(Self {
            path,
            scale,
            anim_fps,
            linear_anim_mapping,
            anim_mapping_dir,
        })
    }
}

// Panics if string does not contain at least 2 elements divided by a whitespace
fn str_to_vec2(s: &str) -> Result<Vec2<u32>, ParseIntError> {
    let mut iter = s.split_whitespace();

    let x = u32::from_str_radix(iter.next().unwrap(), 10)?;
    let y = u32::from_str_radix(iter.next().unwrap(), 10)?;

    Ok(Vec2::new(x, y))
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
struct ZMatExtra {
    detail_tex_scale: f32,
    force_occluder: u8,
    environment_mapping: u8,
    env_mapping_strength: f32,
    wave_mode: u8,
    wave_speed: u8,
    wave_max_amplitude: f32,
    wave_grid_size: f32,
    ignore_sun: u8,
    aplha_func: u8,
}
