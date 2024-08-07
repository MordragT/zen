use std::fmt;

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Render Formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum ZTexFormat {
    /// 32-bit ARGB pixel ColorType with alpha, using 8 bits per channel
    B8G8R8A8 = 0,
    /// 32-bit ARGB pixel ColorType with alpha, using 8 bits per channel
    R8G8B8A8 = 1,
    /// 32-bit ARGB pixel ColorType with alpha, using 8 bits per channel
    A8B8G8R8 = 2,
    /// 32-bit ARGB pixel ColorType with alpha, using 8 bits per channel
    A8R8G8B8 = 3,
    /// 24-bit RGB pixel ColorType with 8 bits per channel
    B8G8R8 = 4,
    /// 4-bit RGB pixel ColorType with 8 bits per channel
    R8G8B8 = 5,
    /// 16-bit ARGB pixel ColorType with 4 bits for each channel
    A4R4G4B4 = 6,
    /// 16-bit pixel ColorType where 5 bits are reserved for each color and 1 bit is reserved for alpha
    A1R5G5B5 = 7,
    /// 16-bit RGB pixel ColorType with 5 bits for red, 6 bits for green, and 5 bits for blue
    R5G6B5 = 8,
    /// 8-bit color indexed
    P8 = 9,
    /// RGB + optional 1 bit alpha: 0.5 byte/px
    DXT1 = 10,
    DXT2 = 11,
    /// RGB + 4 bit alpha: 1 byte/px
    DXT3 = 12,
    DXT4 = 13,
    /// RGBA: 1 byte/px
    DXT5 = 14,
}

impl fmt::Display for ZTexFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::B8G8R8A8 => write!(f, "BGRA8"),
            Self::R8G8B8A8 => write!(f, "RGBA8"),
            Self::A8B8G8R8 => write!(f, "ABGR8"),
            Self::A8R8G8B8 => write!(f, "ARGB8"),
            Self::B8G8R8 => write!(f, "BGR8"),
            Self::R8G8B8 => write!(f, "RGB8"),
            Self::A4R4G4B4 => write!(f, "ARGB4"),
            Self::A1R5G5B5 => write!(f, "A1RGB5"),
            Self::R5G6B5 => write!(f, "R5G6B5"),
            Self::P8 => write!(f, "P8"),
            Self::DXT1 => write!(f, "DXT1"),
            Self::DXT2 => write!(f, "DXT2"),
            Self::DXT3 => write!(f, "DXT3"),
            Self::DXT4 => write!(f, "DXT4"),
            Self::DXT5 => write!(f, "DXT5"),
        }
    }
}
