use std::fmt::Debug;

use math_macro::make_vec;
use my_render::render;
#[allow(unused)]
use my_render::window_mgr;

#[make_vec(x, y, z)]
pub struct Vec33 {}

pub struct Vec333<T: Copy + PartialEq<T>> {
    x: T,
}
impl <T: Copy + Debug + PartialEq<T>> core::fmt::Display for Vec333<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("< Vec333 x: {:?} >", self.x))
    }
}

fn main() {
    render::do_render();
    // window_mgr::create_window("abc".to_string());
    println!("Hello, world!");
}
