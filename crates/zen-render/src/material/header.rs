use serde::{Deserialize, Serialize};
use zen_core::GameKind;

use super::{ZMatError, ZMatResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub(crate) struct ZMatHeader {
    pub name: String,
    pub chunk_size: u32,
    pub version: u16,
    pub object_index: u32,
    pub chunk_name: String,
    pub class_name: String,
}

impl ZMatHeader {
    pub const MATERIAL_VERSION_G1: u16 = 0x4400;
    pub const MATERIAL_VERSION_G2: u16 = 0x9C03;

    pub fn validate(&self) -> ZMatResult<()> {
        if self.kind() == GameKind::Unknown {
            return Err(ZMatError::UnknownVersion(self.version));
        }

        Ok(())
    }

    pub fn kind(&self) -> GameKind {
        if self.version == Self::MATERIAL_VERSION_G1 {
            GameKind::Gothic1
        } else if self.version == Self::MATERIAL_VERSION_G2 {
            GameKind::Gothic2
        } else {
            GameKind::Unknown
        }
    }
}
