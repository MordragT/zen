use serde::{Deserialize, Serialize};
use zen_parser::binary::{BinaryDeserializer, BinaryRead, BinaryResult};

use super::header::ZenMaterialHeader;
use super::ZenMaterialKind;
use crate::math::Vec2;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ZenMaterial {
    kind: ZenMaterialKind,
    color: u32,
    texture: String,
}

impl ZenMaterial {
    pub fn new(handle: impl BinaryRead) -> BinaryResult<Self> {
        let mut deser = BinaryDeserializer::from(handle);
        let header = ZenMaterialHeader::deserialize(&mut deser)?;

        let (kind, color, texture) = if header.is_gothic2() {
            let props = MaterialPropertiesExtended::deserialize(&mut deser)?;

            (props.kind, props.color, props.texture)
        } else {
            let props = MaterialProperties::deserialize(&mut deser)?;

            (props.kind, props.color, props.texture)
        };

        Ok(Self {
            kind,
            color,
            texture,
        })
    }

    pub fn kind(&self) -> ZenMaterialKind {
        self.kind
    }

    pub fn color(&self) -> u32 {
        self.color
    }

    pub fn texture(&self) -> &String {
        &self.texture
    }

    pub fn compiled_texture(&self) -> String {
        let (name, _end) = self
            .texture
            .split_once('.')
            .expect("Every texture has an ending");
        format!("{name}-C.TEX")
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[allow(unused)]
struct MaterialProperties {
    name: String,
    kind: ZenMaterialKind,
    color: u32,
    smooth_angle: f32,
    texture: String,
    tex_scale: String,
    tex_ani_fps: f32,
    tex_ani_map_mode: u8,
    tex_ani_map_dir: String,
    no_coll_det: bool,
    no_light_map: bool,
    load_dont_collapse: u8,
    detail_object: String,
    default_mapping: Vec2<f32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[allow(unused)]
struct MaterialPropertiesExtended {
    name: String,
    kind: ZenMaterialKind,
    color: u32,
    smooth_angle: f32,
    texture: String,
    tex_scale: String,
    tex_ani_fps: f32,
    tex_ani_map_mode: u8,
    tex_ani_map_dir: String,
    no_coll_det: bool,
    no_light_map: bool,
    load_dont_collapse: u8,
    detail_object: String,
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
    default_mapping: Vec2<f32>,
}

// #[cfg(feature = "bevy")]
// mod bevy {
//     use bevy::{color::Color, pbr::StandardMaterial};
//     use std::cmp;

//     use super::ZenMaterial;

//     fn to_color(num: u32) -> Color {
//         let layer = |i| {
//             cmp::max(
//                 0,
//                 cmp::min(1, 3 * i32::abs(1 - 2 * (((num as i32) - i / 3) % 2)) - 1),
//             )
//         };
//         Color::srgba(layer(0) as f32, layer(1) as f32, layer(2) as f32, 1.0)
//     }

//     impl From<ZenMaterial> for StandardMaterial {
//         fn from(zmat: ZenMaterial) -> Self {
//             let color = to_color(zmat.color);

//         }
//     }
// }
