use crate::math::Float2;
use serde::Deserialize;

#[repr(u8)]
#[derive(Deserialize, Debug)]
pub enum Group {
    Undef,
    Metal,
    Stone,
    Wood,
    Earth,
    Water,
    Snow,
}

pub enum Material<'a> {
    Basic(&'a mut BasicMaterial),
    Advanced(&'a mut AdvancedMaterial),
}

/// Materials that are used in Gothic 1
#[derive(Deserialize, Debug)]
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
    default_mapping: Float2,
}

/// Materials used in Gothic 2
#[derive(Deserialize, Debug)]
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
    default_mapping: Float2,
}
