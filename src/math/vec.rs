use math_macro::make_vec;

#[make_vec(x, y)]
pub struct Vec2 {}

#[make_vec(x, y, z)]
pub struct Vec3 {}

#[make_vec(x, y, z, w)]
pub struct Vec4 {}

pub type Vec2i = Vec2<isize>;
pub type Vec2f = Vec2<f64>;
pub type Vec3i = Vec3<isize>;
pub type Vec3f = Vec3<f64>;
