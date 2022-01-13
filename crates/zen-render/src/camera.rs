use hecs::{PreparedQuery, World};
use ultraviolet::{projection, Isometry3, Mat4, Vec3};

#[derive(Debug)]
pub struct Projection {
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[derive(Debug)]
pub struct Camera {
    pub eye: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            eye: Vec3::new(0.0, 2.0, 2.0),
            direction: Vec3::one(),
            up: Vec3::unit_y(),
        }
    }

    pub fn mat(&self) -> Mat4 {
        Mat4::look_at(self.eye, self.eye + self.direction, self.up)
    }
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

    pub fn mat(&self) -> Mat4 {
        projection::perspective_wgpu_dx(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ProjectionViewUniform {
    pub projection: Mat4,
    pub view: Mat4,
}

impl ProjectionViewUniform {
    pub fn new() -> Self {
        Self {
            projection: Mat4::identity(),
            view: Mat4::identity(),
        }
    }
    // pub fn update_view_proj(&mut self, camera: &Isometry3, projection: &Projection) {
    //     self.view_proj = (projection.calc_matrix() * camera.into_homogeneous_matrix());
    // }
}
