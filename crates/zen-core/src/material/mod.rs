//! This crate enables you to convert different materials,
//! that are used in gothic 1 or 2 to a general basic material.

//! You first have to deserialize [BasicMaterial] or [AdvancedMaterial],
//! to use the [TryFrom] implementation.

use crate::{
    archive::Vdfs,
    math::{Vec2, Vec3},
    texture::ZenTexture,
};
use bevy::{
    prelude::{Color, Handle, Image, Material},
    reflect::{TypeUuid, Uuid},
    render::render_resource::{AsBindGroup, ShaderRef},
};
pub use error::MaterialError;
use error::MaterialResult;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::{cmp, convert::TryFrom, fs::File, io::Cursor};
use zen_types::path::INSTANCE;

mod error;

pub const GOTHIC2: u16 = 39939;

/// Simple Material with texture and color
#[derive(TypeUuid, Clone)]
#[uuid = "5c5462ea-1986-11ed-9f7c-233969708b10"]
pub struct ZenMaterial {
    // TODO add support for normal map
    // #[uniform(0)]
    pub color: Color,
    pub metallic: f32,
    pub roughness: f32,
    pub reflectance: f32,
    // #[texture(1)]
    // #[sampler(2)]
    pub texture: Handle<ZenTexture>,
}

// impl Material for ZenMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "zen_material.wgsl".into()
//     }
// }

pub fn to_color(num: u32) -> Color {
    let layer = |i| {
        cmp::max(
            0,
            cmp::min(1, 3 * i32::abs(1 - 2 * (((num as i32) - i / 3) % 2)) - 1),
        )
    };
    Color::Rgba {
        red: layer(0) as f32,
        green: layer(1) as f32,
        blue: layer(2) as f32,
        alpha: 1.0,
    }
}

fn tex_scale_to_vec(scale_str: &str) -> Vec2<u32> {
    let first_str = scale_str.split_whitespace().next().unwrap();
    let first = u32::from_str_radix(first_str, 10).unwrap();
    let second_str = scale_str.split_whitespace().next().unwrap();
    let second = u32::from_str_radix(second_str, 10).unwrap();
    Vec2::new(first, second)
}

/// Materials that are used in Gothic 1
#[derive(Deserialize, Debug, Clone)]
pub struct BasicMaterial {
    name: String,
    group: Group,
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

impl BasicMaterial {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn group(&self) -> &Group {
        &self.group
    }

    pub fn color(&self) -> u32 {
        self.color
    }

    pub fn texture(&self) -> &String {
        &self.texture
    }

    pub fn compiled_texture(&self) -> String {
        let (name, end) = self
            .texture
            .split_once('.')
            .expect("Every texture has an ending");
        format!("{name}-C.TEX")
    }
}

impl From<AdvancedMaterial> for BasicMaterial {
    fn from(am: AdvancedMaterial) -> Self {
        BasicMaterial {
            name: am.name,
            group: am.group,
            color: am.color,
            smooth_angle: am.smooth_angle,
            texture: am.texture,
            tex_scale: am.tex_scale,
            tex_ani_fps: am.tex_ani_fps,
            tex_ani_map_mode: am.tex_ani_map_mode,
            tex_ani_map_dir: am.tex_ani_map_dir,
            no_coll_det: am.no_coll_det,
            no_light_map: am.no_light_map,
            load_dont_collapse: am.load_dont_collapse,
            detail_object: am.detail_object,
            default_mapping: am.default_mapping,
        }
    }
}

/// Materials used in Gothic 2
#[derive(Deserialize, Debug, Clone)]
pub struct AdvancedMaterial {
    name: String,
    group: Group,
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

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Group {
    Undef,
    Metal,
    Stone,
    Wood,
    Earth,
    Water,
    Snow,
}

#[derive(Deserialize, Debug)]
pub struct ChunkHeader {
    pub chunk_size: u32,
    pub version: u16,
    pub object_index: u32,
}
