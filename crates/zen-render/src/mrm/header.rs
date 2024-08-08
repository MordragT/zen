use serde::{Deserialize, Serialize};

use super::{MrmError, MrmResult};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct MrmHeader {
    pub id: u16,
    pub length: u32,
    pub version: u16,
    pub size: u32,
}

impl MrmHeader {
    const PROG_MESH: u16 = 45312;
    //const PROG_MESH_END: u16 = 45567;

    pub(crate) fn validate(&self) -> MrmResult<()> {
        if self.id != Self::PROG_MESH {
            Err(MrmError::UnexpectedHeaderId(self.id))
        } else {
            Ok(())
        }
    }

    pub(crate) fn chunk_length(&self) -> u64 {
        self.length as u64 - std::mem::size_of::<u16>() as u64 - std::mem::size_of::<u32>() as u64
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Offset {
    pub offset: u32,
    pub size: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Offsets {
    pub position: Offset,
    pub normal: Offset,
}

#[derive(Deserialize, Debug)]
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
