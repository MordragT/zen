//! This crate enables you to convert different materials,
//! that are used in gothic 1 or 2 to a general basic material.

//! You first have to deserialize [BasicMaterial] or [AdvancedMaterial],
//! to use the [TryFrom] implementation.

pub use error::Error;
use error::Result;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::{cmp, convert::TryFrom, fs::File, io::Cursor, path::PathBuf};
use zen_archive::Vdfs;
use zen_math::{Vec2, Vec3};
use zen_texture::{ColorType, Texture};
use zen_types::path::{FILES_INSTANCE, INSTANCE};

mod error;

pub const GOTHIC2: u16 = 39939;

/// Simple Material with texture and color
pub struct Material {
    pub texture: PathBuf,
    pub color: Vec3<f32>,
}

impl TryFrom<&GeneralMaterial> for Material {
    type Error = Error;
    /// Creates a simple Material from Materials used in Gothic 1 and 2
    fn try_from(mat: &GeneralMaterial) -> Result<Material> {
        let vdfs_file = File::open(INSTANCE.textures())?;
        let vdfs = Vdfs::new(vdfs_file)?;
        let texture_name = match mat.get_texture().split('.').next() {
            Some(name) => name,
            None => return Err(Error::WrongTextureNameFormat),
        };
        let texture_entry = match vdfs.get_by_name_slice(texture_name) {
            Some(entry) => entry,
            None => return Err(Error::ExpectedValidTextureName(texture_name.to_owned())),
        };
        let texture_data = Cursor::new(texture_entry.data);
        let texture = Texture::from_ztex(texture_data)?;

        let mut texture_name = match texture_entry.name.split('.').next() {
            Some(name) => name.to_string(),
            None => return Err(Error::ExpectedValidTextureName(texture_entry.name)),
        };
        texture_name.push_str(".jpeg");
        let texture_path = FILES_INSTANCE.textures.join(texture_name);
        let output_jpeg = File::create(&texture_path)?;
        texture.to_png(output_jpeg)?;

        let color = to_rgb(mat.get_color());
        Ok(Self {
            texture: texture_path,
            color,
        })
    }
}

fn to_rgb(num: u32) -> Vec3<f32> {
    let layer = |i| {
        cmp::max(
            0,
            cmp::min(1, 3 * i32::abs(1 - 2 * (((num as i32) - i / 3) % 2)) - 1),
        )
    };
    Vec3::new(layer(0) as f32, layer(1) as f32, layer(2) as f32)
}

/// Holds Materials from Gothic 1 and 2
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

impl Into<GeneralMaterial> for BasicMaterial {
    fn into(self) -> GeneralMaterial {
        GeneralMaterial::Basic(self)
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

impl Into<GeneralMaterial> for AdvancedMaterial {
    fn into(self) -> GeneralMaterial {
        GeneralMaterial::Advanced(self)
    }
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
