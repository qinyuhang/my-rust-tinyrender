use crate::math::Vec2i;
use crate::render::line::line;
use image::{ImageBuffer, Pixel};
use std::mem::swap;
use std::ops::{Deref, DerefMut};

pub fn triangle<P, C>(t0: Vec2i, t1: Vec2i, t2: Vec2i, img: &mut ImageBuffer<P, C>, color: P)
where
    P: Pixel<Subpixel = u8> + 'static,
    C: Deref<Target = [u8]> + DerefMut<Target = [u8]>,
{
    line(t0, t1, img, color);
    line(t1, t2, img, color);
    line(t2, t0, img, color);
}
