use ultraviolet::{Similarity3, Vec3};

pub trait Transform {}

// impl Transform for Similarity3 {
//     #[inline]
//     fn local_x(&self) -> Vec3 {
//         self.rotation * Vec3::unit_x()
//     }

//     #[inline]
//     fn local_y(&self) -> Vec3 {
//         self.rotation * Vec3::unit_y()
//     }

//     #[inline]
//     fn local_z(&self) -> Vec3 {
//         self.rotation * Vec3::unit_z()
//     }
// }
