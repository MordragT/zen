use self::ztex::ZenTextureFormat;
use crate::archive::Entry;
use bevy::{
    prelude::Image,
    reflect::TypeUuid,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
pub use error::TextureError;
use error::TextureResult;
use image::{ColorType, ImageEncoder};
use serde::Deserialize;
use std::{
    cmp,
    fmt::Debug,
    io::{Seek, SeekFrom},
};
use texpresso::Format;
use zen_parser::prelude::BinaryDeserializer;

mod error;
mod ztex;

/// A texture with a RGBA8 format
#[derive(TypeUuid, Clone)]
#[uuid = "8aa0408e-865d-473f-e212-9f07a5da5bce"]
pub struct ZenTexture {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    pub name: String,
}

impl Debug for ZenTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Width: {}, Height: {}, Data Length: {}",
            self.name,
            self.width,
            self.height,
            self.pixels.len()
        )
    }
}

impl From<ZenTexture> for Image {
    fn from(tex: ZenTexture) -> Self {
        Image::new(
            Extent3d {
                width: tex.width,
                height: tex.height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            tex.pixels,
            TextureFormat::Rgba8Unorm,
        )
    }
}

impl ZenTexture {
    pub fn new(width: u32, height: u32, pixels: Vec<u8>, name: String) -> Self {
        Self {
            width,
            height,
            pixels,
            name,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.pixels.as_slice()
    }

    pub fn encode<E: ImageEncoder>(&self, encoder: E) -> TextureResult<()> {
        encoder.write_image(&self.pixels, self.width, self.height, ColorType::Rgba8)?;
        Ok(())
    }
}

impl<'a> TryFrom<Entry<'a>> for ZenTexture {
    type Error = TextureError;
    /// Convert ZTEX to Texture
    fn try_from(entry: Entry) -> TextureResult<Self> {
        let name = entry.name().to_owned();

        let mut deserializer = BinaryDeserializer::from(entry);
        let header = ztex::Header::deserialize(&mut deserializer)?;
        if header.signature() != ztex::FILE_SIGNATURE || header.version() != ztex::FILE_VERSION {
            return Err(TextureError::WrongSignature);
        }

        let (width, height) = header.dimensions();
        let size = width * height;

        let mipmap_count = cmp::max(1, header.mipmap_level());
        let mut size_of_all_mip_maps = 0;
        for layer in 0..mipmap_count {
            size_of_all_mip_maps += get_mip_map_size(header.format(), width, height, layer);
        }
        let size_of_biggest_mip_map = get_mip_map_size(header.format(), width, height, 0);
        let pos_of_biggest_mip_map = size_of_all_mip_maps - size_of_biggest_mip_map;
        deserializer.seek(SeekFrom::Current(pos_of_biggest_mip_map as i64))?;

        log::debug!("Decoding texture with format: {:?}", header.format());
        let pixels = match header.format() {
            ZenTextureFormat::B8G8R8A8 => {
                let mut pixels = vec![0_u8; 4 * size as usize];
                for chunk in pixels.chunks_mut(4) {
                    // bgra
                    let mut pixel = <[u8; 4]>::deserialize(&mut deserializer)?;
                    // abgr
                    pixel.rotate_left(3);
                    // rgba
                    pixel.reverse();
                    chunk.copy_from_slice(&pixel);
                }
                pixels
            }
            ZenTextureFormat::R8G8B8A8 => {
                deserializer.len_queue.push(4 * size as usize);
                let pixels = <Vec<u8>>::deserialize(&mut deserializer)?;
                pixels
            }
            ZenTextureFormat::A8B8G8R8 => {
                let mut pixels = vec![0_u8; 4 * size as usize];
                for chunk in pixels.chunks_mut(4) {
                    let mut pixel = <[u8; 4]>::deserialize(&mut deserializer)?;
                    pixel.reverse();
                    chunk.copy_from_slice(&pixel);
                }
                pixels
            }
            ZenTextureFormat::A8R8G8B8 => {
                let mut pixels = vec![0_u8; 4 * size as usize];
                for chunk in pixels.chunks_mut(4) {
                    let mut pixel = <[u8; 4]>::deserialize(&mut deserializer)?;
                    pixel.rotate_left(1);
                    chunk.copy_from_slice(&pixel);
                }
                pixels
            }
            ZenTextureFormat::B8G8R8 => {
                let mut pixels = vec![0_u8; 4 * size as usize];
                for chunk in pixels.chunks_mut(4) {
                    let pixel = <[u8; 3]>::deserialize(&mut deserializer)?;
                    chunk.copy_from_slice(&[pixel[2], pixel[1], pixel[0], 0xff]);
                }
                pixels
            }
            ZenTextureFormat::R8G8B8 => {
                let mut pixels = vec![0_u8; 4 * size as usize];
                for chunk in pixels.chunks_mut(4) {
                    let pixel = <[u8; 3]>::deserialize(&mut deserializer)?;
                    chunk.copy_from_slice(&[pixel[0], pixel[1], pixel[2], 0xff]);
                }
                pixels
            }
            ZenTextureFormat::A4R4G4B4 => {
                let mut pixels = vec![0_u8; 4 * size as usize];
                for chunk in pixels.chunks_mut(4) {
                    let pixel = <u16>::deserialize(&mut deserializer)?;
                    chunk.copy_from_slice(&[
                        ((pixel >> 8) & 0b1111) as u8,  // r
                        ((pixel >> 4) & 0b1111) as u8,  // g
                        (pixel & 0b1111) as u8,         // b
                        ((pixel >> 12) & 0b1111) as u8, // a
                    ]);
                }
                pixels
            }
            ZenTextureFormat::A1R5G5B5 => {
                let mut pixels = vec![0_u8; 4 * size as usize];
                for chunk in pixels.chunks_mut(4) {
                    let pixel = <u16>::deserialize(&mut deserializer)?;
                    chunk.copy_from_slice(&[
                        ((pixel >> 10) & 0b1111_1) as u8, // r
                        ((pixel >> 6) & 0b1111_1) as u8,  // g
                        (pixel & 0b1111_1) as u8,         // b
                        ((pixel >> 15) & 0b1) as u8,      // a
                    ]);
                }
                pixels
            }
            ZenTextureFormat::R5G6B5 => {
                let mut pixels = vec![0_u8; 4 * size as usize];
                for chunk in pixels.chunks_mut(4) {
                    let pixel = <u16>::deserialize(&mut deserializer)?;
                    chunk.copy_from_slice(&[
                        ((pixel >> 11) & 0b1111_1) as u8, // r
                        ((pixel >> 5) & 0b1111_11) as u8, // g
                        (pixel & 0b1111_1) as u8,         // b
                        0xff,                             // a
                    ]);
                }
                pixels
            }
            ZenTextureFormat::P8 => unimplemented!(),
            ZenTextureFormat::DXT1 => {
                deserializer.len_queue.push(size as usize / 2);
                let data = <Vec<u8>>::deserialize(&mut deserializer)?;

                let mut output = vec![0_u8; size as usize * 4];
                Format::Bc1.decompress(&data, width as usize, height as usize, &mut output);
                output
            }
            ZenTextureFormat::DXT2 => {
                todo!()
            }
            ZenTextureFormat::DXT3 => {
                deserializer.len_queue.push(size as usize);
                let data = <Vec<u8>>::deserialize(&mut deserializer)?;

                let mut output = vec![0_u8; size as usize * 4];
                Format::Bc2.decompress(&data, width as usize, height as usize, &mut output);
                output
            }
            ZenTextureFormat::DXT4 => {
                todo!()
            }
            ZenTextureFormat::DXT5 => {
                deserializer.len_queue.push(size as usize);
                let data = <Vec<u8>>::deserialize(&mut deserializer)?;

                let mut output = vec![0_u8; size as usize * 4];
                Format::Bc3.decompress(&data, width as usize, height as usize, &mut output);
                output
            }
        };
        let texture = ZenTexture::new(width, height, pixels, name);
        Ok(texture)
    }
}

/// level 0 = highest, ztex is built other way round, 0 = lowest
fn get_mip_map_size(color_type: ZenTextureFormat, width: u32, height: u32, level: u32) -> u32 {
    let mut x = cmp::max(1, width);
    let mut y = cmp::max(1, height);
    for _ in 0..level {
        if x > 1 {
            x >>= 1;
        }
        if y > 1 {
            y >>= 1;
        }
    }
    match color_type {
        ZenTextureFormat::B8G8R8A8
        | ZenTextureFormat::R8G8B8A8
        | ZenTextureFormat::A8B8G8R8
        | ZenTextureFormat::A8R8G8B8 => x * y * 4,
        ZenTextureFormat::B8G8R8 | ZenTextureFormat::R8G8B8 => x * y * 3,
        ZenTextureFormat::A4R4G4B4 | ZenTextureFormat::A1R5G5B5 | ZenTextureFormat::R5G6B5 => {
            x * y * 2
        }
        ZenTextureFormat::P8 => x * y,
        ZenTextureFormat::DXT1 => cmp::max(1, x / 4) * cmp::max(1, y / 4) * 8,
        ZenTextureFormat::DXT2
        | ZenTextureFormat::DXT3
        | ZenTextureFormat::DXT4
        | ZenTextureFormat::DXT5 => cmp::max(1, x / 4) * cmp::max(1, y / 4) * 16,
    }
}
