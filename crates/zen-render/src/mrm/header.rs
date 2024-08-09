use serde::{Deserialize, Serialize};
use zen_core::GameKind;

use super::{MrmError, MrmResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct MrmHeader {
    pub id: u16,
    pub length: u32,
    pub version: u16,
    pub size: u32,
}

impl MrmHeader {
    const PROG_MESH: u16 = 0xB100;
    //const PROG_MESH_END: u16 = 0xB1FF;

    const MRM_VERSION_G1: u16 = 0x305;
    const MRM_VERSION_G2: u16 = 0x905;

    pub(crate) fn validate(&self) -> MrmResult<()> {
        if self.id != Self::PROG_MESH {
            Err(MrmError::UnexpectedHeaderId(self.id))
        } else if self.kind() == GameKind::Unknown {
            Err(MrmError::UnknownVersion(self.version))
        } else {
            Ok(())
        }
    }

    pub(crate) fn kind(&self) -> GameKind {
        if self.version == Self::MRM_VERSION_G1 {
            GameKind::Gothic1
        } else if self.version == Self::MRM_VERSION_G2 {
            GameKind::Gothic2
        } else {
            GameKind::Unknown
        }
    }

    pub(crate) fn chunk_length(&self) -> u64 {
        self.length as u64 - std::mem::size_of::<u16>() as u64 - std::mem::size_of::<u32>() as u64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub(crate) struct Offset {
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub(crate) struct Offsets {
    pub position: Offset,
    pub normal: Offset,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub(crate) struct SubMeshOffsets {
    pub triangles: Offset,
    pub wedges: Offset,
    pub colors: Offset,
    pub triangle_plane_indices: Offset,
    pub triangle_planes: Offset,
    pub wedge_map: Offset,
    pub vertex_updates: Offset,
    pub triangle_edges: Offset,
    pub edges: Offset,
    pub edge_scores: Offset,
}
