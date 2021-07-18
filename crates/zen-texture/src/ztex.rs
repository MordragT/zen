// use ddsfile::D3DFormat;
// use ddsfile::DxgiFormat;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

// Definitions for compressed ZenGin Textures (.tex)
// Gothic stores its textures in a proprietary format called ZTEX, which is basically DDS
// with a different Header and some minor other modifications.

// Those ZTEX-files are generated by the game by converting TGA files, which can be seen as
// caching mechanism. Therefore, in the other game files, the original TGA-name will be used.

// To still load the correct file, we have to convert the input filename of "SAMPLE.TGA" into
// "SAMPLE-C.TEX", which is the compiled ZTEX file.

// In case there is no such compiled ZTEX file, we will try to load the original TGA file instead.

pub const FILE_SIGNATURE: u32 = 0x5845545A; // 'XETZ' (little-endian)
                                            //const FILE_SIGNATURE: usize = 0x5A544558; // 'ZTEX' (big-endian)
pub const FILE_VERSION: u32 = 0x0;
/// Number of Palellte Entries
pub const PALETTE_ENTRIES: usize = 0x100;
/// Render Formats

#[derive(Deserialize_repr, Eq, PartialEq, Debug, Copy, Clone)]
#[repr(u32)]
pub enum Format {
    B8G8R8A8, // 0, 32-bit ARGB pixel format with alpha, using 8 bits per channel
    R8G8B8A8, // 1, 32-bit ARGB pixel format with alpha, using 8 bits per channel
    A8B8G8R8, // 2, 32-bit ARGB pixel format with alpha, using 8 bits per channel
    A8R8G8B8, // 3, 32-bit ARGB pixel format with alpha, using 8 bits per channel
    B8G8R8,   // 4, 24-bit RGB pixel format with 8 bits per channel
    R8G8B8,   // 5, 24-bit RGB pixel format with 8 bits per channel
    A4R4G4B4, // 6, 16-bit ARGB pixel format with 4 bits for each channel
    A1R5G5B5, // 7, 16-bit pixel format where 5 bits are reserved for each color and 1 bit is reserved for alpha
    R5G6B5, // 8, 16-bit RGB pixel format with 5 bits for red, 6 bits for green, and 5 bits for blue
    P8,     // 9, 8-bit color indexed
    DXT1,   // A, DXT1 compression texture format
    DXT2,   // B, DXT2 compression texture format
    DXT3,   // C, DXT3 compression texture format
    DXT4,   // D, DXT4 compression texture format
    DXT5,   // E, DXT5 compression texture format
}

// impl TryInto<D3DFormat> for Format {
//     type Error = ();
//     fn try_into(self) -> Result<D3DFormat, Self::Error> {
//         match self {
//             Self::B8G8R8A8 => Err(()),
//             Self::R8G8B8A8 => Err(()),
//             Self::A8B8G8R8 => Ok(D3DFormat::A8B8G8R8),
//             Self::A8R8G8B8 => Ok(D3DFormat::A8R8G8B8),
//             Self::B8G8R8 => Err(()),
//             Self::R8G8B8 => Ok(D3DFormat::R8G8B8),
//             Self::A4R4G4B4 => Ok(D3DFormat::A4R4G4B4),
//             Self::A1R5G5B5 => Ok(D3DFormat::A1R5G5B5),
//             Self::R5G6B5 => Ok(D3DFormat::R5G6B5),
//             Self::P8 => Err(()),
//             Self::DXT1 => Ok(D3DFormat::DXT1),
//             Self::DXT2 => Ok(D3DFormat::DXT2),
//             Self::DXT3 => Ok(D3DFormat::DXT3),
//             Self::DXT4 => Ok(D3DFormat::DXT4),
//             Self::DXT5 => Ok(D3DFormat::DXT5),
//         }
//     }
// }

// impl TryInto<DxgiFormat> for Format {
//     type Error = ();
//     fn try_into(self) -> Result<DxgiFormat, Self::Error> {
//         match self {
//             Self::B8G8R8A8 => Ok(DxgiFormat::B8G8R8A8_Typeless),
//             Self::R8G8B8A8 => Ok(DxgiFormat::R8G8B8A8_Typeless),
//             Self::A8B8G8R8 => Err(()),
//             Self::A8R8G8B8 => Err(()),
//             Self::B8G8R8 => Ok(DxgiFormat::B8G8R8X8_Typeless),
//             Self::R8G8B8 => Err(()),
//             Self::A4R4G4B4 => Err(()),
//             Self::A1R5G5B5 => Err(()),
//             Self::R5G6B5 => Err(()),
//             Self::P8 => Ok(DxgiFormat::P8),
//             Self::DXT1 => Err(()),
//             Self::DXT2 => Err(()),
//             Self::DXT3 => Err(()),
//             Self::DXT4 => Err(()),
//             Self::DXT5 => Err(()),
//         }
//     }
// }

/// Info Block
#[derive(Deserialize, Debug)]
pub struct Info {
    format: Format,
    width: u32,        // mipmap 0
    height: u32,       // mipmap 0
    mipmap_level: u32, // 1 = none
    ref_width: u32,    // ingame x
    ref_height: u32,   // ingame y
    avg_color: u32,    // A8R8G8B8
}
/// File Header
#[derive(Deserialize, Debug)]
pub struct Header {
    signature: u32,
    version: u32,
    info: Info,
}

impl Header {
    pub fn format(&self) -> Format {
        self.info.format
    }
    pub fn width(&self) -> u32 {
        self.info.width
    }
    pub fn height(&self) -> u32 {
        self.info.height
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.info.width, self.info.height)
    }
    pub fn signature(&self) -> u32 {
        self.signature
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn mipmap_level(&self) -> u32 {
        self.info.mipmap_level
    }
}

/// Palette Entry
#[derive(Deserialize, Debug)]
pub struct Entry {
    r: u8,
    g: u8,
    b: u8,
}

/// Stored Palette
pub type Palette = Vec<Entry>;

// impl Palette {
//     pub fn new() -> Self {
//         Self { entries: vec![] }
//     }
//     pub fn len(&self) -> usize {
//         self.entries.len()
//     }
//     pub fn push(&mut self, entry: Entry) {
//         self.entries.push(entry);
//     }
// }
