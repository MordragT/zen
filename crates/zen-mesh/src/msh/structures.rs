use crate::structures::Plane;
use serde::Deserialize;
use vek::{Vec2, Vec3};

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
#[derive(Debug)]
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
    pub vertex: u32,
    pub feature: u32,
}

impl<T: Into<u32>> From<IndexPacked<T>> for Index {
    fn from(i: IndexPacked<T>) -> Self {
        Self {
            vertex: i.vertex_index.into(),
            feature: i.feat_index,
        }
    }
}

#[derive(Deserialize)]
#[repr(C, packed(4))]
pub struct FeatureChunk {
    pub tex_coord: Vec2<f32>,
    pub light_stat: u32,
    pub vert_normal: Vec3<f32>,
}

// Data in memory is packed therefor unable to utilize serde Deserialization
pub struct Polygon {
    pub material_index: i16,
    pub light_map_index: i16,
    pub plane: Plane,
    pub flags: PolyFlags,
    pub num_indices: u8,
    pub indices: Vec<Index>,
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
    normal: Vec3<f32>,
}
