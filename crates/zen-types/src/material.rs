use std::fmt::Debug;

use serde::Deserialize;
use serde_repr::Deserialize_repr;
use vek::Vec2;

pub const GOTHIC2: u16 = 39939;

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

// pub trait GeneralMaterial: Debug + Clone + Sized {
//     fn get_color(&self) -> u32;
//     fn get_texture(&self) -> &str;
//     fn get_texture_scale(&self) -> Vec2<u32>;
// }

#[derive(Debug, Clone)]
pub enum GeneralMaterial {
    Basic(BasicMaterial),
    Advanced(AdvancedMaterial),
}

impl GeneralMaterial {
    pub fn get_color(&self) -> u32 {
        match self {
            GeneralMaterial::Basic(b) => b.color,
            GeneralMaterial::Advanced(a) => a.color,
        }
    }
    pub fn get_texture(&self) -> &str {
        match self {
            GeneralMaterial::Basic(b) => &b.texture,
            GeneralMaterial::Advanced(a) => &a.texture,
        }
    }
    pub fn get_texture_scale(&self) -> Vec2<u32> {
        match self {
            GeneralMaterial::Basic(b) => tex_scale_to_vec(&b.tex_scale),
            GeneralMaterial::Advanced(a) => tex_scale_to_vec(&a.tex_scale),
        }
    }
    //pub fn get_texture(&self, )
}

fn tex_scale_to_vec(scale_str: &str) -> Vec2<u32> {
    let first_str = scale_str.split_whitespace().next().unwrap();
    let first = u32::from_str_radix(first_str, 10).unwrap();
    let second_str = scale_str.split_whitespace().next().unwrap();
    let second = u32::from_str_radix(second_str, 10).unwrap();
    Vec2::from((first, second))
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

impl Into<GeneralMaterial> for BasicMaterial {
    fn into(self) -> GeneralMaterial {
        GeneralMaterial::Basic(self)
    }
}

// impl GeneralMaterial for BasicMaterial {
//     fn get_color(&self) -> u32 {
//         self.color
//     }
//     fn get_texture(&self) -> &str {
//         &self.texture
//     }
//     fn get_texture_scale(&self) -> Vec2<u32> {
//         tex_scale_to_vec(&self.tex_scale)
//     }
// }

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

impl Into<GeneralMaterial> for AdvancedMaterial {
    fn into(self) -> GeneralMaterial {
        GeneralMaterial::Advanced(self)
    }
}

// impl GeneralMaterial for AdvancedMaterial {
//     fn get_color(&self) -> u32 {
//         self.color
//     }
//     fn get_texture(&self) -> &str {
//         &self.texture
//     }
//     fn get_texture_scale(&self) -> Vec2<u32> {
//         tex_scale_to_vec(&self.tex_scale)
//     }
// }
