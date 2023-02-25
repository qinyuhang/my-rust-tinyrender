#[allow(unused)]
use math_macro::*;

pub mod matrix;
pub mod quaternion;
pub mod vec;

pub use matrix::*;
pub use quaternion::*;
pub use vec::*;

mod test;
