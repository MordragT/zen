use image::{dds::DdsDecoder, jpeg::JpegEncoder, ImageDecoder};
use std::{cmp, fs::File, io::Cursor, path::PathBuf};
use vek::Vec3;
use zen_archive::Vdfs;
use zen_types::{
    material::GeneralMaterial,
    path::{FILES_INSTANCE, INSTANCE},
};

pub struct Material {
    pub texture: PathBuf,
    pub color: Vec3<f32>,
}

impl From<&GeneralMaterial> for Material {
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
