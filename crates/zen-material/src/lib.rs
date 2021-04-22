use image::{dds::DdsDecoder, jpeg::JpegEncoder, ImageDecoder};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::{cmp, fs::File, io::Cursor, path::PathBuf};
use vek::Vec2;
use vek::Vec3;
use zen_archive::Vdfs;
use zen_types::path::{FILES_INSTANCE, INSTANCE};

pub const GOTHIC2: u16 = 39939;

pub struct Material {
    pub texture: PathBuf,
    pub color: Vec3<f32>,
}

impl From<&GeneralMaterial> for Material {
    /// Creates a simple Material from Materials used in Gothic 1 and 2
    fn from(mat: &GeneralMaterial) -> Material {
        let vdfs_file = File::open(INSTANCE.textures()).unwrap();
        let vdfs = Vdfs::new(vdfs_file).unwrap();
        //println!("Texture: {}", mat.get_texture());
        let texture_name = mat.get_texture().split('.').next().unwrap();
        //vdfs.list();
        let texture_entry = vdfs.get_by_name_slice(texture_name).unwrap();
        let texture_data = Cursor::new(texture_entry.data);
        let dds = zen_texture::convert_ztex_to_dds(texture_data).unwrap();
        let mut dds_file_buf = vec![];
        dds.write(&mut dds_file_buf).unwrap();
        let dds_file = Cursor::new(dds_file_buf);
        let decoder = DdsDecoder::new(dds_file).unwrap();
        let (width, height) = decoder.dimensions();
        let color_type = decoder.color_type();
        let mut dds_bytes = vec![0_u8; decoder.total_bytes() as usize];
        decoder.read_image(&mut dds_bytes).unwrap();

        let mut texture_name = texture_entry.name.split('.').next().unwrap().to_string();
        texture_name.push_str(".jpeg");
        let texture_path = FILES_INSTANCE.textures.join(texture_name);
        let mut output_jpeg = File::create(&texture_path).unwrap();
        let mut encoder = JpegEncoder::new(&mut output_jpeg);
        encoder
            .encode(dds_bytes.as_slice(), width, height, color_type)
            .unwrap();
        //vdf.list();
        let color = to_rgb(mat.get_color());
        //dbg!(mat.get_texture_scale());
        Self {
            texture: texture_path,
            color,
        }
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
