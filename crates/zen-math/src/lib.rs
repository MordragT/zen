use std::ops::Mul;

#[cfg(feature = "serde")]
use serde::Deserialize;

#[cfg(feature = "serde")]
#[derive(Default, Deserialize, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Vec2<T: Copy + PartialOrd>([T; 2]);

#[cfg(not(feature = "serde"))]
#[derive(Default, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Vec2<T: Copy + PartialOrd>([T; 2]);

impl<T: Copy + PartialOrd> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }

    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn min_x(&mut self, x: T) {
        if self.x() > x {
            self.0[0] = x;
        }
    }

    pub fn max_x(&mut self, x: T) {
        if self.x() < x {
            self.0[0] = x;
        }
    }

    pub fn min_y(&mut self, y: T) {
        if self.y() > y {
            self.0[1] = y;
        }
    }

    pub fn max_y(&mut self, y: T) {
        if self.y() < y {
            self.0[1] = y;
        }
    }

    pub fn yx(&self) -> Self {
        Self([self.y(), self.x()])
    }

    pub fn to_array(self) -> [T; 2] {
        self.0
    }

    pub fn to_vec(self) -> Vec<T> {
        self.to_array().to_vec()
    }
}

impl<T: Copy + PartialOrd> From<[T; 2]> for Vec2<T> {
    fn from(arr: [T; 2]) -> Self {
        Self(arr)
    }
}

impl<T: Copy + PartialOrd + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self([self.x() * rhs, self.y() * rhs])
    }
}

#[cfg(feature = "serde")]
#[derive(Default, Deserialize, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Vec3<T: Copy + PartialOrd>([T; 3]);

#[cfg(not(feature = "serde"))]
#[derive(Default, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Vec3<T: Copy + PartialOrd>([T; 3]);

impl<T: Copy + PartialOrd> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn z(&self) -> T {
        self.0[2]
    }

    /// Changes the vector to the minimum numbers of both vectors
    pub fn min(&mut self, other: &Vec3<T>) {
        self.min_x(other.x());
        self.min_y(other.y());
        self.min_z(other.z());
    }

    /// Changes the vector to the maximum numbers of both vectors
    pub fn max(&mut self, other: &Vec3<T>) {
        self.max_x(other.x());
        self.max_y(other.y());
        self.max_z(other.z());
    }

    pub fn min_x(&mut self, x: T) {
        if self.x() > x {
            self.0[0] = x;
        }
    }

    pub fn max_x(&mut self, x: T) {
        if self.x() < x {
            self.0[0] = x;
        }
    }

    pub fn min_y(&mut self, y: T) {
        if self.y() > y {
            self.0[1] = y;
        }
    }

    pub fn max_y(&mut self, y: T) {
        if self.y() < y {
            self.0[1] = y;
        }
    }

    pub fn min_z(&mut self, z: T) {
        if self.z() > z {
            self.0[2] = z;
        }
    }

    pub fn max_z(&mut self, z: T) {
        if self.z() < z {
            self.0[2] = z;
        }
    }

    pub fn xy(&self) -> Vec2<T> {
        Vec2([self.x(), self.y()])
    }

    pub fn yx(&self) -> Vec2<T> {
        Vec2([self.y(), self.x()])
    }

    pub fn xz(&self) -> Vec2<T> {
        Vec2([self.x(), self.z()])
    }

    pub fn zx(&self) -> Vec2<T> {
        Vec2([self.z(), self.x()])
    }

    pub fn yz(&self) -> Vec2<T> {
        Vec2([self.y(), self.z()])
    }

    pub fn zy(&self) -> Vec2<T> {
        Vec2([self.z(), self.y()])
    }

    pub fn to_array(self) -> [T; 3] {
        self.0
    }

    pub fn to_vec(self) -> Vec<T> {
        self.to_array().to_vec()
    }
}

impl<T: Copy + PartialOrd> From<[T; 3]> for Vec3<T> {
    fn from(arr: [T; 3]) -> Self {
        Self(arr)
    }
}

impl<T: Copy + PartialOrd + Mul<Output = T>> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self([self.x() * rhs, self.y() * rhs, self.z() * rhs])
    }
}

#[cfg(feature = "serde")]
#[derive(Default, Deserialize, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Vec4<T: Copy + PartialOrd>([T; 4]);

#[cfg(not(feature = "serde"))]
#[derive(Default, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Vec4<T: Copy + PartialOrd>([T; 4]);

impl<T: Copy + PartialOrd> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }

    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn z(&self) -> T {
        self.0[2]
    }

    pub fn w(&self) -> T {
        self.0[3]
    }

    pub fn min_x(&mut self, x: T) {
        if self.x() > x {
            self.0[0] = x;
        }
    }

    pub fn max_x(&mut self, x: T) {
        if self.x() < x {
            self.0[0] = x;
        }
    }

    pub fn min_y(&mut self, y: T) {
        if self.y() > y {
            self.0[1] = y;
        }
    }

    pub fn max_y(&mut self, y: T) {
        if self.y() < y {
            self.0[1] = y;
        }
    }

    pub fn min_z(&mut self, z: T) {
        if self.z() > z {
            self.0[2] = z;
        }
    }

    pub fn max_z(&mut self, z: T) {
        if self.z() < z {
            self.0[2] = z;
        }
    }

    pub fn min_w(&mut self, w: T) {
        if self.w() > w {
            self.0[3] = w;
        }
    }

    pub fn max_w(&mut self, w: T) {
        if self.w() < w {
            self.0[3] = w;
        }
    }

    pub fn xy(&self) -> Vec2<T> {
        Vec2([self.x(), self.y()])
    }

    pub fn yx(&self) -> Vec2<T> {
        Vec2([self.y(), self.x()])
    }

    pub fn xz(&self) -> Vec2<T> {
        Vec2([self.x(), self.z()])
    }

    pub fn zx(&self) -> Vec2<T> {
        Vec2([self.z(), self.x()])
    }

    pub fn xw(&self) -> Vec2<T> {
        Vec2([self.x(), self.w()])
    }

    pub fn wx(&self) -> Vec2<T> {
        Vec2([self.w(), self.x()])
    }

    pub fn yz(&self) -> Vec2<T> {
        Vec2([self.y(), self.z()])
    }

    pub fn zy(&self) -> Vec2<T> {
        Vec2([self.z(), self.y()])
    }

    pub fn yw(&self) -> Vec2<T> {
        Vec2([self.y(), self.w()])
    }

    pub fn wy(&self) -> Vec2<T> {
        Vec2([self.w(), self.y()])
    }

    pub fn zw(&self) -> Vec2<T> {
        Vec2([self.z(), self.w()])
    }

    pub fn wz(&self) -> Vec2<T> {
        Vec2([self.w(), self.z()])
    }

    pub fn xyz(&self) -> Vec3<T> {
        Vec3([self.x(), self.y(), self.z()])
    }

    pub fn yzw(&self) -> Vec3<T> {
        Vec3([self.y(), self.z(), self.w()])
    }

    pub fn zwx(&self) -> Vec3<T> {
        Vec3([self.z(), self.w(), self.x()])
    }

    pub fn wxy(&self) -> Vec3<T> {
        Vec3([self.w(), self.x(), self.y()])
    }

    pub fn to_array(self) -> [T; 4] {
        self.0
    }

    pub fn to_vec(self) -> Vec<T> {
        self.to_array().to_vec()
    }
}

impl<T: Copy + PartialOrd> From<[T; 4]> for Vec4<T> {
    fn from(arr: [T; 4]) -> Self {
        Self(arr)
    }
}

impl<T: Copy + PartialOrd + Mul<Output = T>> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self([
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
            self.w() * rhs,
        ])
    }
}
