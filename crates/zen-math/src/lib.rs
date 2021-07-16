#[repr(transparent)]
pub struct Vec2<T: Copy>([T; 2]);

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }

    pub fn x(&self) -> T {
        self.0[0]
    }

    pub fn y(&self) -> T {
        self.0[1]
    }

    pub fn yx(&self) -> Self {
        Self([self.y(), self.x()])
    }

    pub fn to_array(self) -> [T; 2] {
        self.0
    }
}

impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(arr: [T; 2]) -> Self {
        Self(arr)
    }
}

#[repr(transparent)]
pub struct Vec3<T: Copy>([T; 3]);

impl<T: Copy> Vec3<T> {
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
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(arr: [T; 3]) -> Self {
        Self(arr)
    }
}

#[repr(transparent)]
pub struct Vec4<T: Copy>([T; 4]);

impl<T: Copy> Vec4<T> {
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
}

impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(arr: [T; 4]) -> Self {
        Self(arr)
    }
}
