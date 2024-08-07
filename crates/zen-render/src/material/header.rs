use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub(crate) struct ZenMaterialHeader {
    pub name: String,
    pub chunk_size: u32,
    pub version: u16,
    pub object_index: u32,
    pub chunk_name: String,
    pub class_name: String,
}

impl ZenMaterialHeader {
    pub const GOTHIC2_VERSION: u16 = 39939;

    pub fn is_gothic2(&self) -> bool {
        self.version == Self::GOTHIC2_VERSION
    }
}
