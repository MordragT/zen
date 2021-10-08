pub use first_person::*;
use hecs::{PreparedQuery, World};
use ultraviolet::{projection, Isometry3, Mat4};

mod first_person;

pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Mat4 {
        projection::perspective_reversed_z_wgpu_dx_gl(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

pub type Camera = Isometry3;

pub fn calc_matrix(world: &mut World, query: &mut PreparedQuery<&mut Camera>) -> Mat4 {
    for (_id, camera) in query.query_mut(world) {
        // Mat4::look_at(
        //     isometry.translation,
        //     Vec3::new(self.yaw.cos(), self.pitch.sin(), self.yaw.sin()).normalize(),
        //     Vec3::Y,
        // )
        return Isometry3::into_homogeneous_matrix(*camera);
    }
    Mat4::identity()
}

// pub struct Camera {
//     pub left: f32,
//     pub right: f32,
//     pub top: f32,
//     pub bottom: f32,
//     pub near: f32,
//     pub far: f32,
// }
