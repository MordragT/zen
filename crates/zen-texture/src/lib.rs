use serde::Deserialize;
use std::cmp;
use std::fmt;
use std::io::SeekFrom;
use zen_parser::prelude::{BinaryDeserializer, BinaryRead};
use ztex::Format;

//pub use ddsfile;

mod ztex;

/// Error Type
#[derive(Debug)]
pub enum Error {
    WrongSignature,
    ConversionError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongSignature => f.write_str("Wrong ZTEX Signature or Version"),
            Self::ConversionError => f.write_str("Couldnt convert ZTEX format to DDS format."),
        }
    }
}

impl std::error::Error for Error {}

pub enum TextureFormat {
    RGBA8,
    BGRA8,
    RGBA16,
    RGB8,
}

pub struct Texture {
    width: u32,
    height: u32,
    format: TextureFormat,
    pixels: Vec<u8>,
}

impl Texture {
    pub fn new(width: u32, height: u32, format: TextureFormat, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            format,
            pixels,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn format(&self) -> &TextureFormat {
        &self.format
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.pixels.as_slice()
    }

    /// Convert ZTEX to Texture
    pub fn from_ztex<'a, R: BinaryRead>(reader: R) -> Result<Self, Error> {
        let mut deserializer = BinaryDeserializer::from(reader);
        let header = ztex::Header::deserialize(&mut deserializer).unwrap();
        if header.signature() != ztex::FILE_SIGNATURE || header.version() != ztex::FILE_VERSION {
            return Err(Error::WrongSignature);
        }

        // let _palette = match header.get_format() == ztex::Format::P8 {
        //     true => {
        //         deserializer.len_queue.push(ztex::PALETTE_ENTRIES);
        //         Some(ztex::Palette::deserialize(&mut deserializer).unwrap())
        //         // let mut palette = ztex::Palette::new();
        //         // for _ in 0..ztex::PALETTE_ENTRIES {
        //         //     let entry =
        //         //         bincode::deserialize_from::<&mut Cursor<&[u8]>, ztex::Entry>(&mut reader)
        //         //             .unwrap();
        //         //     palette.push(entry);
        //         // }
        //         // match palette.len() == ztex::PALETTE_ENTRIES {
        //         //     true => Some(palette),
        //         //     false => None,
        //         // }
        //     }
        //     false => None,
        // };

        let (width, height) = header.dimensions();

        let mipmap_count = cmp::max(1, header.mipmap_level());
        let mut size_of_all_mip_maps = 0;
        for layer in 0..mipmap_count {
            size_of_all_mip_maps += get_mip_map_size(&header.format(), width, height, layer);
        }
        let size_of_biggest_mip_map = get_mip_map_size(&header.format(), width, height, 0);
        let pos_of_biggest_mip_map = size_of_all_mip_maps - size_of_biggest_mip_map;
        deserializer
            .parser
            .seek(SeekFrom::Current(pos_of_biggest_mip_map as i64))
            .unwrap();

        let size = width * height;

        let texture = match header.format() {
            Format::B8G8R8A8 => {
                let pixels = (0..size)
                    .map(|_| <[u8; 4]>::deserialize(&mut deserializer).unwrap())
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::BGRA8, pixels)
            }
            Format::R8G8B8A8 => {
                let pixels = (0..size)
                    .map(|_| <[u8; 4]>::deserialize(&mut deserializer).unwrap())
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::RGBA8, pixels)
            }
            Format::A8B8G8R8 => {
                let pixels = (0..size)
                    .map(|_| {
                        let mut pixel = <[u8; 4]>::deserialize(&mut deserializer).unwrap();
                        pixel.reverse();
                        pixel
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::RGBA8, pixels)
            }
            Format::A8R8G8B8 => {
                let pixels = (0..size)
                    .map(|_| {
                        let mut pixel = <[u8; 4]>::deserialize(&mut deserializer).unwrap();
                        pixel.reverse();
                        pixel
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::BGRA8, pixels)
            }
            Format::B8G8R8 => {
                let pixels = (0..size)
                    .map(|_| {
                        let pixel = <[u8; 3]>::deserialize(&mut deserializer).unwrap();
                        [pixel[0], pixel[1], pixel[2], 0xff]
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::BGRA8, pixels)
            }
            Format::R8G8B8 => {
                let pixels = (0..size)
                    .map(|_| {
                        let pixel = <[u8; 3]>::deserialize(&mut deserializer).unwrap();
                        [pixel[0], pixel[1], pixel[2], 0xff]
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::RGBA8, pixels)
            }
            Format::A4R4G4B4 => {
                let pixels = (0..size)
                    .map(|_| {
                        let pixel = <u16>::deserialize(&mut deserializer).unwrap();
                        [
                            ((pixel >> 8) & 0b1111) as u8,  // r
                            ((pixel >> 4) & 0b1111) as u8,  // g
                            (pixel & 0b1111) as u8,         // b
                            ((pixel >> 12) & 0b1111) as u8, // a
                        ]
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::RGBA8, pixels)
            }
            Format::A1R5G5B5 => {
                let pixels = (0..size)
                    .map(|_| {
                        let pixel = <u16>::deserialize(&mut deserializer).unwrap();
                        [
                            ((pixel >> 10) & 0b1111_1) as u8, // r
                            ((pixel >> 6) & 0b1111_1) as u8,  // g
                            (pixel & 0b1111_1) as u8,         // b
                            ((pixel >> 15) & 0b1) as u8,      // a
                        ]
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::RGBA8, pixels)
            }
            Format::R5G6B5 => {
                let pixels = (0..size)
                    .map(|_| {
                        let pixel = <u16>::deserialize(&mut deserializer).unwrap();
                        [
                            ((pixel >> 11) & 0b1111_1) as u8, // r
                            ((pixel >> 5) & 0b1111_11) as u8, // g
                            (pixel & 0b1111_1) as u8,         // b
                            0xff,                             // a
                        ]
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                Texture::new(width, height, TextureFormat::RGBA8, pixels)
            }
            Format::P8 => unimplemented!(),
            Format::DXT1 => {
                let mut decoded = vec![0_u8; size as usize * 3];
                decoded
                    .chunks_mut((width / 4 * 48) as usize)
                    .for_each(|chunk| {
                        deserializer.len_queue.push((width / 4 * 8) as usize);
                        decode_dxt1_row(
                            <Vec<u8>>::deserialize(&mut deserializer)
                                .unwrap()
                                .as_slice(),
                            chunk,
                        );
                    });
                Texture::new(width, height, TextureFormat::RGB8, decoded)
            }
            Format::DXT2 => unimplemented!(),
            Format::DXT3 => {
                let mut decoded = vec![0_u8; size as usize * 4];
                decoded
                    .chunks_mut((width / 4 * 64) as usize)
                    .for_each(|chunk| {
                        deserializer.len_queue.push((width / 4 * 16) as usize);
                        decode_dxt3_row(
                            <Vec<u8>>::deserialize(&mut deserializer)
                                .unwrap()
                                .as_slice(),
                            chunk,
                        );
                    });
                Texture::new(width, height, TextureFormat::RGBA8, decoded)
            }
            Format::DXT4 => unimplemented!(),
            Format::DXT5 => {
                let mut decoded = vec![0_u8; size as usize * 4];
                decoded
                    .chunks_mut((width / 4 * 64) as usize)
                    .for_each(|chunk| {
                        deserializer.len_queue.push((width / 4 * 16) as usize);
                        decode_dxt5_row(
                            <Vec<u8>>::deserialize(&mut deserializer)
                                .unwrap()
                                .as_slice(),
                            chunk,
                        );
                    });
                Texture::new(width, height, TextureFormat::RGBA8, decoded)
            }
        };
        Ok(texture)
    }
}
/// level 0 = highest, ztex is built other way round, 0 = lowest
fn get_mip_map_size(format: &ztex::Format, width: u32, height: u32, level: u32) -> u32 {
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
    match format {
        ztex::Format::B8G8R8A8
        | ztex::Format::R8G8B8A8
        | ztex::Format::A8B8G8R8
        | ztex::Format::A8R8G8B8 => x * y * 4,
        ztex::Format::B8G8R8 | ztex::Format::R8G8B8 => x * y * 3,
        ztex::Format::A4R4G4B4 | ztex::Format::A1R5G5B5 | ztex::Format::R5G6B5 => x * y * 2,
        ztex::Format::P8 => x * y,
        ztex::Format::DXT1 => cmp::max(1, x / 4) * cmp::max(1, y / 4) * 8,
        ztex::Format::DXT2 | ztex::Format::DXT3 | ztex::Format::DXT4 | ztex::Format::DXT5 => {
            cmp::max(1, x / 4) * cmp::max(1, y / 4) * 16
        }
    }
}

/// Constructs the DXT5 alpha lookup table from the two alpha entries
/// if alpha0 > alpha1, constructs a table of [a0, a1, 6 linearly interpolated values from a0 to a1]
/// if alpha0 <= alpha1, constructs a table of [a0, a1, 4 linearly interpolated values from a0 to a1, 0, 0xFF]
fn alpha_table_dxt5(alpha0: u8, alpha1: u8) -> [u8; 8] {
    let mut table = [alpha0, alpha1, 0, 0, 0, 0, 0, 0xFF];
    if alpha0 > alpha1 {
        for i in 2..8u16 {
            table[i as usize] =
                (((8 - i) * u16::from(alpha0) + (i - 1) * u16::from(alpha1)) / 7) as u8;
        }
    } else {
        for i in 2..6u16 {
            table[i as usize] =
                (((6 - i) * u16::from(alpha0) + (i - 1) * u16::from(alpha1)) / 5) as u8;
        }
    }
    table
}

// The following stuff is borrowed from: https://github.com/image-rs/image/blob/master/src/codecs/dxt.rs

type Rgb = [u8; 3];

/// decodes a 5-bit R, 6-bit G, 5-bit B 16-bit packed color value into 8-bit RGB
/// mapping is done so min/max range values are preserved. So for 5-bit
/// values 0x00 -> 0x00 and 0x1F -> 0xFF
fn enc565_decode(value: u16) -> Rgb {
    let red = (value >> 11) & 0x1F;
    let green = (value >> 5) & 0x3F;
    let blue = (value) & 0x1F;
    [
        (red * 0xFF / 0x1F) as u8,
        (green * 0xFF / 0x3F) as u8,
        (blue * 0xFF / 0x1F) as u8,
    ]
}

/// decodes an 8-byte dxt color block into the RGB channels of a 16xRGB or 16xRGBA block.
/// source should have a length of 8, dest a length of 48 (RGB) or 64 (RGBA)
fn decode_dxt_colors(source: &[u8], dest: &mut [u8], is_dxt1: bool) {
    // sanity checks, also enable the compiler to elide all following bound checks
    assert!(source.len() == 8 && (dest.len() == 48 || dest.len() == 64));
    // calculate pitch to store RGB values in dest (3 for RGB, 4 for RGBA)
    let pitch = dest.len() / 16;

    // extract color data
    let color0 = u16::from(source[0]) | (u16::from(source[1]) << 8);
    let color1 = u16::from(source[2]) | (u16::from(source[3]) << 8);
    let color_table = u32::from(source[4])
        | (u32::from(source[5]) << 8)
        | (u32::from(source[6]) << 16)
        | (u32::from(source[7]) << 24);
    // let color_table = source[4..8].iter().rev().fold(0, |t, &b| (t << 8) | b as u32);

    // decode the colors to rgb format
    let mut colors = [[0; 3]; 4];
    colors[0] = enc565_decode(color0);
    colors[1] = enc565_decode(color1);

    // determine color interpolation method
    if color0 > color1 || !is_dxt1 {
        // linearly interpolate the other two color table entries
        for i in 0..3 {
            colors[2][i] = ((u16::from(colors[0][i]) * 2 + u16::from(colors[1][i]) + 1) / 3) as u8;
            colors[3][i] = ((u16::from(colors[0][i]) + u16::from(colors[1][i]) * 2 + 1) / 3) as u8;
        }
    } else {
        // linearly interpolate one other entry, keep the other at 0
        for i in 0..3 {
            colors[2][i] = ((u16::from(colors[0][i]) + u16::from(colors[1][i]) + 1) / 2) as u8;
        }
    }

    // serialize the result. Every color is determined by looking up
    // two bits in color_table which identify which color to actually pick from the 4 possible colors
    for i in 0..16 {
        dest[i * pitch..i * pitch + 3]
            .copy_from_slice(&colors[(color_table >> (i * 2)) as usize & 3]);
    }
}

/// Decodes a 16-byte block of dxt5 data to a 16xRGBA block
fn decode_dxt5_block(source: &[u8], dest: &mut [u8]) {
    assert!(source.len() == 16 && dest.len() == 64);

    // extract alpha index table (stored as little endian 64-bit value)
    let alpha_table = source[2..8]
        .iter()
        .rev()
        .fold(0, |t, &b| (t << 8) | u64::from(b));

    // alhpa level decode
    let alphas = alpha_table_dxt5(source[0], source[1]);

    // serialize alpha
    for i in 0..16 {
        dest[i * 4 + 3] = alphas[(alpha_table >> (i * 3)) as usize & 7];
    }

    // handle colors
    decode_dxt_colors(&source[8..16], dest, false);
}

/// Decodes a 16-byte block of dxt3 data to a 16xRGBA block
fn decode_dxt3_block(source: &[u8], dest: &mut [u8]) {
    assert!(source.len() == 16 && dest.len() == 64);

    // extract alpha index table (stored as little endian 64-bit value)
    let alpha_table = source[0..8]
        .iter()
        .rev()
        .fold(0, |t, &b| (t << 8) | u64::from(b));

    // serialize alpha (stored as 4-bit values)
    for i in 0..16 {
        dest[i * 4 + 3] = ((alpha_table >> (i * 4)) as u8 & 0xF) * 0x11;
    }

    // handle colors
    decode_dxt_colors(&source[8..16], dest, false);
}

/// Decodes a 8-byte block of dxt1 data to a 16xRGB block
fn decode_dxt1_block(source: &[u8], dest: &mut [u8]) {
    assert!(source.len() == 8 && dest.len() == 48);
    decode_dxt_colors(&source, dest, true);
}

/// Decode a row of DXT1 data to four rows of RGB data.
/// source.len() should be a multiple of 8, otherwise this panics.
fn decode_dxt1_row(source: &[u8], dest: &mut [u8]) {
    assert!(source.len() % 8 == 0);
    let block_count = source.len() / 8;
    assert!(dest.len() >= block_count * 48);

    // contains the 16 decoded pixels per block
    let mut decoded_block = [0u8; 48];

    for (x, encoded_block) in source.chunks(8).enumerate() {
        decode_dxt1_block(encoded_block, &mut decoded_block);

        // copy the values from the decoded block to linewise RGB layout
        for line in 0..4 {
            let offset = (block_count * line + x) * 12;
            dest[offset..offset + 12].copy_from_slice(&decoded_block[line * 12..(line + 1) * 12]);
        }
    }
}

/// Decode a row of DXT3 data to four rows of RGBA data.
/// source.len() should be a multiple of 16, otherwise this panics.
fn decode_dxt3_row(source: &[u8], dest: &mut [u8]) {
    assert!(source.len() % 16 == 0);
    let block_count = source.len() / 16;
    assert!(dest.len() >= block_count * 64);

    // contains the 16 decoded pixels per block
    let mut decoded_block = [0u8; 64];

    for (x, encoded_block) in source.chunks(16).enumerate() {
        decode_dxt3_block(encoded_block, &mut decoded_block);

        // copy the values from the decoded block to linewise RGB layout
        for line in 0..4 {
            let offset = (block_count * line + x) * 16;
            dest[offset..offset + 16].copy_from_slice(&decoded_block[line * 16..(line + 1) * 16]);
        }
    }
}

/// Decode a row of DXT5 data to four rows of RGBA data.
/// source.len() should be a multiple of 16, otherwise this panics.
fn decode_dxt5_row(source: &[u8], dest: &mut [u8]) {
    assert!(source.len() % 16 == 0);
    let block_count = source.len() / 16;
    assert!(dest.len() >= block_count * 64);

    // contains the 16 decoded pixels per block
    let mut decoded_block = [0u8; 64];

    for (x, encoded_block) in source.chunks(16).enumerate() {
        decode_dxt5_block(encoded_block, &mut decoded_block);

        // copy the values from the decoded block to linewise RGB layout
        for line in 0..4 {
            let offset = (block_count * line + x) * 16;
            dest[offset..offset + 16].copy_from_slice(&decoded_block[line * 16..(line + 1) * 16]);
        }
    }
}
