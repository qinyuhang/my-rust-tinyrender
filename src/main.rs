use std::fmt::Debug;

use math_macro::make_vec;
use my_render::render;
#[allow(unused)]
use my_render::window_mgr;

#[make_vec(x, y, z)]
pub struct Vec33 {}

fn main() {
    render::do_render();
    // window_mgr::create_window("abc".to_string());
    println!("Hello, world!");
    let v = Vec33::new(1, 2, 3);
    let vv = v;
    assert_eq!(vv, v);
    println!("{}", v);
    println!("{}", vv);
}
