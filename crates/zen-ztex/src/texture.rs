use std::{
    fmt,
    io::{self, SeekFrom},
};

use serde::Deserialize;
use zen_parser::binary::{BinaryDeserializer, BinaryRead};

use crate::{
    error::{ZTexError, ZTexResult},
    format::ZTexFormat,
    header::ZTexHeader,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZTex<H> {
    header: ZTexHeader,
    handle: H,
    offset: u64,
}

impl<H> ZTex<H> {
    /// Returns width of the biggest mip map
    pub fn width(&self) -> u32 {
        self.header.width
    }

    /// Returns height of the biggest mip map
    pub fn height(&self) -> u32 {
        self.header.height
    }

    pub fn size(&self) -> u32 {
        self.header.width * self.header.height
    }

    pub fn format(&self) -> ZTexFormat {
        self.header.format
    }
}

impl<H: BinaryRead> ZTex<H> {
    pub fn new(mut handle: H) -> ZTexResult<Self> {
        let mut deser = BinaryDeserializer::from(&mut handle);

        let header = ZTexHeader::deserialize(&mut deser)?;
        header.validate()?;

        let offset = handle.stream_position()?;

        Ok(Self {
            header,
            handle,
            offset,
        })
    }

    pub fn fetch_mut(&mut self, level: u32) -> io::Result<Vec<u8>> {
        let pos = self.header.mip_map_pos(level) as u64;
        let size = self.header.mip_map_size(level) as usize;

        self.handle.seek(SeekFrom::Start(self.offset + pos))?;

        let mut pixels = vec![0; size];
        self.handle.read_exact(&mut pixels)?;

        Ok(pixels)
    }
}

impl<H> fmt::Display for ZTex<H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.header.fmt(f)
    }
}

#[cfg(feature = "encode")]
impl<H: BinaryRead> ZTex<H> {
    pub fn encode<E: image::ImageEncoder>(&mut self, encoder: E) -> image::ImageResult<()> {
        use texpresso::Format;

        let mut pixels = self.fetch_mut(0)?;
        let size = self.size() as usize;

        let (pixels, color_type) = match self.format() {
            ZTexFormat::B8G8R8A8 => (pixels, image::ExtendedColorType::Bgra8),
            ZTexFormat::R8G8B8A8 => (pixels, image::ExtendedColorType::Rgba8),
            ZTexFormat::A8B8G8R8 => {
                for pixel in pixels.chunks_mut(4) {
                    pixel.reverse();
                }
                (pixels, image::ExtendedColorType::Rgba8)
            }
            ZTexFormat::A8R8G8B8 => {
                for pixel in pixels.chunks_mut(4) {
                    pixel.rotate_left(1);
                }
                (pixels, image::ExtendedColorType::Rgba8)
            }
            ZTexFormat::B8G8R8 => (pixels, image::ExtendedColorType::Bgr8),

            ZTexFormat::R8G8B8 => (pixels, image::ExtendedColorType::Rgb8),

            ZTexFormat::A4R4G4B4 => {
                let mut rgba = vec![0; 4 * size];

                for (argb, rgba) in pixels.array_chunks::<2>().zip(rgba.chunks_mut(4)) {
                    let argb = u16::from_le_bytes(*argb);
                    rgba.copy_from_slice(&[
                        ((argb >> 8) & 0b1111) as u8,  // r
                        ((argb >> 4) & 0b1111) as u8,  // g
                        (argb & 0b1111) as u8,         // b
                        ((argb >> 12) & 0b1111) as u8, // a
                    ]);
                }
                (rgba, image::ExtendedColorType::Rgba8)
            }
            ZTexFormat::A1R5G5B5 => {
                let mut rgba = vec![0; 4 * size];

                for (argb, rgba) in pixels.array_chunks::<2>().zip(rgba.chunks_mut(4)) {
                    let argb = u16::from_le_bytes(*argb);
                    rgba.copy_from_slice(&[
                        ((argb >> 10) & 0b1111_1) as u8, // r
                        ((argb >> 6) & 0b1111_1) as u8,  // g
                        (argb & 0b1111_1) as u8,         // b
                        ((argb >> 15) & 0b1) as u8,      // a
                    ]);
                }
                (rgba, image::ExtendedColorType::Rgba8)
            }
            ZTexFormat::R5G6B5 => {
                let mut rgba = vec![0; 4 * size];

                for (rgb, rgba) in pixels.array_chunks::<2>().zip(rgba.chunks_mut(4)) {
                    let rgb = u16::from_le_bytes(*rgb);
                    rgba.copy_from_slice(&[
                        ((rgb >> 11) & 0b1111_1) as u8, // r
                        ((rgb >> 5) & 0b1111_11) as u8, // g
                        (rgb & 0b1111_1) as u8,         // b
                        0xff,                           // a
                    ]);
                }
                (rgba, image::ExtendedColorType::Rgba8)
            }
            ZTexFormat::P8 => unimplemented!(),
            ZTexFormat::DXT1 => {
                let width = self.width() as usize;
                let height = self.height() as usize;
                let mut rgba = vec![0; 4 * size];

                Format::Bc1.decompress(&pixels, width, height, &mut rgba);
                (rgba, image::ExtendedColorType::Rgba8)
            }
            ZTexFormat::DXT2 => {
                todo!()
            }
            ZTexFormat::DXT3 => {
                let width = self.width() as usize;
                let height = self.height() as usize;
                let mut rgba = vec![0; 4 * size];

                Format::Bc2.decompress(&pixels, width, height, &mut rgba);
                (rgba, image::ExtendedColorType::Rgba8)
            }
            ZTexFormat::DXT4 => {
                todo!()
            }
            ZTexFormat::DXT5 => {
                let width = self.width() as usize;
                let height = self.height() as usize;
                let mut rgba = vec![0; 4 * size];

                Format::Bc3.decompress(&pixels, width, height, &mut rgba);
                (rgba, image::ExtendedColorType::Rgba8)
            }
        };

        encoder.write_image(&pixels, self.width(), self.height(), color_type)
    }
}

#[cfg(feature = "bevy")]
impl<H: BinaryRead> TryFrom<ZTex<H>> for bevy::prelude::Image {
    type Error = ZTexError;

    fn try_from(mut ztex: ZTex<H>) -> Result<Self, Self::Error> {
        use bevy::{
            prelude::Image,
            render::render_asset::RenderAssetUsages,
            render::render_resource::{Extent3d, TextureDimension, TextureFormat},
        };

        let mut pixels = ztex.fetch_mut(0)?;
        let size = ztex.size() as usize;

        let (pixels, format) = match ztex.format() {
            ZTexFormat::B8G8R8A8 => (pixels, TextureFormat::Bgra8Unorm),
            ZTexFormat::R8G8B8A8 => (pixels, TextureFormat::Rgba8Unorm),
            ZTexFormat::A8B8G8R8 => {
                for pixel in pixels.chunks_mut(4) {
                    pixel.reverse();
                }
                (pixels, TextureFormat::Rgba8Unorm)
            }
            ZTexFormat::A8R8G8B8 => {
                for pixel in pixels.chunks_mut(4) {
                    pixel.rotate_left(1);
                }
                (pixels, TextureFormat::Rgba8Unorm)
            }
            ZTexFormat::B8G8R8 => {
                let mut bgra = vec![0; 4 * size];

                for (bgr, bgra) in pixels.chunks(3).zip(bgra.chunks_mut(4)) {
                    bgra.copy_from_slice(&[bgr, &[0xff]].concat());
                }
                (bgra, TextureFormat::Bgra8Unorm)
            }
            ZTexFormat::R8G8B8 => {
                let mut rgba = vec![0; 4 * size];

                for (rgb, rgba) in pixels.chunks(3).zip(rgba.chunks_mut(4)) {
                    rgba.copy_from_slice(&[rgb, &[0xff]].concat());
                }
                (rgba, TextureFormat::Rgba8Unorm)
            }
            ZTexFormat::A4R4G4B4 => {
                let mut rgba = vec![0; 4 * size];

                for (argb, rgba) in pixels.array_chunks::<2>().zip(rgba.chunks_mut(4)) {
                    let argb = u16::from_le_bytes(*argb);
                    rgba.copy_from_slice(&[
                        ((argb >> 8) & 0b1111) as u8,  // r
                        ((argb >> 4) & 0b1111) as u8,  // g
                        (argb & 0b1111) as u8,         // b
                        ((argb >> 12) & 0b1111) as u8, // a
                    ]);
                }
                (rgba, TextureFormat::Rgba8Unorm)
            }
            ZTexFormat::A1R5G5B5 => {
                let mut rgba = vec![0; 4 * size];

                for (argb, rgba) in pixels.array_chunks::<2>().zip(rgba.chunks_mut(4)) {
                    let argb = u16::from_le_bytes(*argb);
                    rgba.copy_from_slice(&[
                        ((argb >> 10) & 0b1111_1) as u8, // r
                        ((argb >> 6) & 0b1111_1) as u8,  // g
                        (argb & 0b1111_1) as u8,         // b
                        ((argb >> 15) & 0b1) as u8,      // a
                    ]);
                }
                (rgba, TextureFormat::Rgba8Unorm)
            }
            ZTexFormat::R5G6B5 => {
                let mut rgba = vec![0; 4 * size];

                for (rgb, rgba) in pixels.array_chunks::<2>().zip(rgba.chunks_mut(4)) {
                    let rgb = u16::from_le_bytes(*rgb);
                    rgba.copy_from_slice(&[
                        ((rgb >> 11) & 0b1111_1) as u8, // r
                        ((rgb >> 5) & 0b1111_11) as u8, // g
                        (rgb & 0b1111_1) as u8,         // b
                        0xff,                           // a
                    ]);
                }
                (rgba, TextureFormat::Rgba8Unorm)
            }
            ZTexFormat::P8 => unimplemented!(),
            ZTexFormat::DXT1 => (pixels, TextureFormat::Bc1RgbaUnorm),
            ZTexFormat::DXT2 => {
                todo!()
            }
            ZTexFormat::DXT3 => (pixels, TextureFormat::Bc2RgbaUnorm),
            ZTexFormat::DXT4 => {
                todo!()
            }
            ZTexFormat::DXT5 => (pixels, TextureFormat::Bc3RgbaUnorm),
        };

        let image = Image::new(
            Extent3d {
                width: ztex.width(),
                height: ztex.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            pixels,
            format,
            RenderAssetUsages::RENDER_WORLD,
        );

        Ok(image)
    }
}
