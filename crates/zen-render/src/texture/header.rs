use serde::{Deserialize, Serialize};
use std::{cmp, fmt};

use super::{
    error::{ZTexError, ZTexResult},
    format::ZTexFormat,
};

/// File Header
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub(crate) struct ZTexHeader {
    pub signature: [u8; 4],
    pub version: u32,
    pub format: ZTexFormat,
    pub width: u32,        // mipmap 0
    pub height: u32,       // mipmap 0
    pub mipmap_level: u32, // 1 = none
    pub ref_width: u32,    // ingame x
    pub ref_height: u32,   // ingame y
    pub avg_color: u32,    // A8R8G8B8
}

impl ZTexHeader {
    const FILE_SIGNATURE: [u8; 4] = *b"ZTEX";
    const FILE_VERSION: u32 = 0x0;

    pub fn validate(&self) -> ZTexResult<()> {
        if self.signature != Self::FILE_SIGNATURE {
            Err(ZTexError::WrongSignature)
        } else if self.version != Self::FILE_VERSION {
            Err(ZTexError::WrongVersion)
        } else {
            Ok(())
        }
    }

    pub fn mip_map_count(&self) -> u32 {
        cmp::max(1, self.mipmap_level)
    }

    pub fn mip_map_pos(&self, level: u32) -> u32 {
        let range = (level + 1)..self.mip_map_count();
        range.map(|layer| self.mip_map_size(layer)).sum()
    }

    /// normally level 0 = highest, ztex is built other way round, 0 = lowest
    pub fn mip_map_size(&self, level: u32) -> u32 {
        let mut x = cmp::max(1, self.width);
        let mut y = cmp::max(1, self.height);

        for _ in 0..level {
            if x > 1 {
                x >>= 1;
            }
            if y > 1 {
                y >>= 1;
            }
        }

        match self.format {
            ZTexFormat::B8G8R8A8
            | ZTexFormat::R8G8B8A8
            | ZTexFormat::A8B8G8R8
            | ZTexFormat::A8R8G8B8 => x * y * 4,
            ZTexFormat::B8G8R8 | ZTexFormat::R8G8B8 => x * y * 3,
            ZTexFormat::A4R4G4B4 | ZTexFormat::A1R5G5B5 | ZTexFormat::R5G6B5 => x * y * 2,
            ZTexFormat::P8 => x * y,
            ZTexFormat::DXT1 => cmp::max(1, x / 4) * cmp::max(1, y / 4) * 8,
            ZTexFormat::DXT2 | ZTexFormat::DXT3 | ZTexFormat::DXT4 | ZTexFormat::DXT5 => {
                cmp::max(1, x / 4) * cmp::max(1, y / 4) * 16
            }
        }
    }
}

impl fmt::Display for ZTexHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            signature,
            version,
            format,
            width,
            height,
            mipmap_level,
            ref_width: _,
            ref_height: _,
            avg_color: _,
        } = self;

        write!(f, "Signature: {}\n", String::from_utf8_lossy(signature))?;
        write!(f, "Version: {version}\n")?;
        write!(f, "Format: {format}\n")?;
        write!(f, "Width: {width}\n")?;
        write!(f, "Height: {height}\n")?;
        write!(f, "Mip-Map-Level: {mipmap_level}\n")?;

        Ok(())
    }
}
