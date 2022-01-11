use hecs::{PreparedQuery, World};
use ultraviolet::{projection, Isometry3, Mat4};

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
        projection::perspective_wgpu_dx(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

pub type Camera = Isometry3;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ProjectionUniform {
    view_proj: Mat4,
}

impl ProjectionUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::identity(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_proj = (projection.calc_matrix() * camera.into_homogeneous_matrix());
    }
}
