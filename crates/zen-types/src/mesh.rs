use crate::material::Material;
use serde::Deserialize;

#[derive(Deserialize)]
#[repr(C, packed(4))]
pub struct Date {
    year: u32,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
}

// impl fmt::Display for Date {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}.{}.{}, {}:{}:{}",
//             self.day, self.month, self.year, self.hour, self.minute, self.second
//         )
//     }
// }
/// Information about one of the chunks in a zen-file
pub struct ChunkHeader {
    start_position: u32,
    size: u32,
    verison: u16,
    object_id: u32,
    name: String,
    class_name: String,
    create_object: bool,
}

#[derive(Deserialize)]
#[repr(C, packed(4))]
pub struct Chunk {
    pub id: u16,
    pub length: u32,
}

#[derive(Deserialize)]
#[repr(C, packed(4))]
pub struct FeatureChunk {
    uv: (f32, f32),
    light_stat: u32,
    vert_normal: (f32, f32, f32),
}

#[derive(Deserialize, Debug)]
pub struct Plane {
    distance: f32,
    normal: (f32, f32, f32),
}

impl From<PlanePacked> for Plane {
    fn from(p: PlanePacked) -> Self {
        Self {
            distance: p.distance,
            normal: p.normal,
        }
    }
}

#[derive(Deserialize)]
#[repr(C, packed(4))]
pub struct PlanePacked {
    distance: f32,
    normal: (f32, f32, f32),
}

// gothic 2 flags = 24 bits
#[derive(Deserialize, Debug)]
#[repr(packed(4))]
pub struct PolyGothicTwoFlags {
    data: u8,
    pub sector_index: u16,
}

impl PolyGothicTwoFlags {
    pub fn new(data: u8, sector_index: u16) -> Self {
        Self { data, sector_index }
    }
    pub fn get_portal_poly(&self) -> u8 {
        self.data & 0b11 // ersten 2 bits
    }
    pub fn occluder(&self) -> bool {
        match self.data & 0b100 {
            0 => false,
            1 => true,
            _ => unimplemented!(),
        }
    }
    pub fn sector_poly(&self) -> bool {
        match self.data & 0b1000 {
            0 => false,
            1 => true,
            _ => unimplemented!(),
        }
    }
    pub fn must_relight(&self) -> bool {
        match self.data & 0b10000 {
            0 => false,
            1 => true,
            _ => unimplemented!(),
        }
    }
    pub fn portal_indoor_outdoor(&self) -> bool {
        match self.data & 0b100000 {
            0 => false,
            1 => true,
            _ => unimplemented!(),
        }
    }
    pub fn ghost_occluder(&self) -> bool {
        match self.data & 0b1000000 {
            0 => false,
            1 => true,
            _ => unimplemented!(),
        }
    }
    pub fn no_dyn_light_near(&self) -> bool {
        match self.data & 0b10000000 {
            0 => false,
            1 => true,
            _ => unimplemented!(),
        }
    }
}

// Gothic 1 currently has 25 bits for the poly flags which as of date
// 08.2020 is not able to be accomplished in rust
// #[repr(packed(4))]
// #[derive(Deserialize)]
// pub struct PolyGothic1Flags {
//     data: u16,
//     sector_index: u16,
// }

// 8 + 16 + 16 + 128 + flags: Gothic1=25, Gothic2=24
// #[repr(packed(1))]
// #[derive(Deserialize)]
// pub struct PolygonPacked {
//     material_index: i16,
//     light_map_index: i16,
//     plane: PlanePacked,
//     flags: PolyGothic2Flags,
//     num_vertices: u8,
// }

// TODO: gothic 1 hat eigentlich 1 bit mehr an flags daher deserialize eig nicht wünschenswert
#[derive(Deserialize)]
pub enum PolyFlags {
    Gothic2(PolyGothicTwoFlags),
}

impl From<PolyGothicTwoFlags> for PolyFlags {
    fn from(f: PolyGothicTwoFlags) -> Self {
        Self::Gothic2(f)
    }
}

#[derive(Deserialize)]
#[repr(C, packed(4))]
pub struct IndexPacked<T: Into<u32>> {
    vertex_index: T,
    feat_index: u32,
}

#[derive(Debug)]
pub struct Index {
    vertex_index: u32,
    feat_index: u32,
}

impl<T: Into<u32>> From<IndexPacked<T>> for Index {
    fn from(i: IndexPacked<T>) -> Self {
        Self {
            vertex_index: i.vertex_index.into(),
            feat_index: i.feat_index,
        }
    }
}

// Data in memory is packed therefor unable to utilize serde Deserialization
pub struct Polygon {
    material_index: i16,
    light_map_index: i16,
    plane: Plane,
    flags: PolyFlags,
    num_indices: u8,
    indices: Vec<Index>,
}

impl Polygon {
    pub fn new(
        material_index: i16,
        light_map_index: i16,
        plane: Plane,
        flags: PolyFlags,
        num_indices: u8,
        indices: Vec<Index>,
    ) -> Self {
        Self {
            material_index,
            light_map_index,
            plane,
            flags,
            num_indices,
            indices,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32),
    tex_coord: (f32, f32),
    color: u32,
}

#[derive(Deserialize, Debug)]
pub struct Triangle {
    flags: PolyGothicTwoFlags,
    light_map_index: i16,
    vertices: [Vertex; 3],
    submesh_index: i16,
}
#[derive(Deserialize, Debug)]
pub struct DataEntry {
    pub offset: u32,
    pub size: u32,
}
#[derive(Deserialize, Debug)]
pub struct Offset {
    pub position: DataEntry,
    pub normal: DataEntry,
}
#[derive(Deserialize, Debug)]
pub struct SubMeshOffsets {
    pub triangles: DataEntry,
    pub wedges: DataEntry,
    pub colors: DataEntry,
    pub triangle_plane_indices: DataEntry,
    pub triangle_planes: DataEntry,
    pub wedge_map: DataEntry,
    pub vertex_updates: DataEntry,
    pub triangle_edges: DataEntry,
    pub edges: DataEntry,
    pub edge_scores: DataEntry,
}

#[derive(Deserialize, Debug)]
pub struct Wedge {
    normal: (f32, f32, f32),
    tex_coord: (f32, f32),
    vertex_index: u16,
}

#[derive(Debug)]
pub struct SubMesh {
    material: Material,
    triangles: Vec<Triangle>,
    wedges: Vec<Wedge>,
    colors: Vec<f32>,
    triangle_plane_indices: Vec<u16>,
    triangle_planes: Vec<Plane>,
    triangle_edges: Vec<(u16, u16, u16)>,
    wedge_map: Vec<u16>,
    edges: Vec<(u16, u16)>,
    edge_scores: Vec<f32>,
}

impl SubMesh {
    pub fn new(
        material: Material,
        triangles: Vec<Triangle>,
        wedges: Vec<Wedge>,
        colors: Vec<f32>,
        triangle_plane_indices: Vec<u16>,
        triangle_planes: Vec<Plane>,
        triangle_edges: Vec<(u16, u16, u16)>,
        wedge_map: Vec<u16>,
        edges: Vec<(u16, u16)>,
        edge_scores: Vec<f32>,
    ) -> Self {
        Self {
            material,
            triangles,
            wedges,
            colors,
            triangle_plane_indices,
            triangle_planes,
            triangle_edges,
            wedge_map,
            edges,
            edge_scores,
        }
    }
}
