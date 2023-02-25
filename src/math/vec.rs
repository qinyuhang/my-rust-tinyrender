use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Clone, Copy)]
// x, y, z, w
pub struct Vec4<
    T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(PartialEq, Clone, Copy)]
// x, y, z
pub struct Vec3<
    T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(PartialEq, Clone, Copy)]
// x, y
pub struct Vec2<
    T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
> {
    pub x: T,
    pub y: T,
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Vec4<T>
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4 { x, y, z, w }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Add for Vec4<T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Sub for Vec4<T>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Mul<T> for Vec4<T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Vec3<T>
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Add for Vec3<T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Sub for Vec3<T>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Mul<T> for Vec3<T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Vec2<T>
{
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Add for Vec2<T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Sub for Vec2<T>
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<
        T: Copy + PartialEq + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Mul<T> for Vec2<T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub type Vec2i = Vec2<isize>;
pub type Vec2f = Vec2<f64>;
pub type Vec3i = Vec3<isize>;
pub type Vec3f = Vec3<f64>;
