use crate::math::Vec2i;
use image::{ImageBuffer, Pixel};
use std::mem::swap;
use std::ops::{Deref, DerefMut};

pub fn line<P, C>(s: Vec2i, e: Vec2i, img: &mut ImageBuffer<P, C>, color: P)
where
    P: Pixel<Subpixel = u8> + 'static,
    C: Deref<Target = [u8]> + DerefMut<Target = [u8]>,
{
    // attempt 3
    let mut steep = false;
    let mut s = s.clone();
    let mut e = e.clone();
    if (s.x - e.x).abs() < (s.y - e.y).abs() {
        steep = true;
        swap(&mut s.x, &mut s.y);
        swap(&mut e.x, &mut e.y);
    }
    if s.x > e.x {
        swap(&mut s.x, &mut e.x);
        swap(&mut s.y, &mut e.y);
    }
    // attempt 2
    let delta_x = e.x - s.x;
    let delta_y = e.y - s.y;
    let derror2 = delta_y.abs() * 2;
    let mut error2 = 0;
    let mut x = s.x;
    let mut y = s.y;
    while x <= e.x {
        if steep {
            img.put_pixel(y as u32, x as u32, color);
        } else {
            img.put_pixel(x as u32, y as u32, color);
        }
        x += 1;
        // error += dydx;
        error2 += derror2;
        if error2 > delta_x {
            y += if e.y > s.y { 1 } else { -1 };
            error2 -= delta_x * 2;
        }
    }
    // attempt 1
    // let mut t = 0.1f64;
    // while (t < 1.0f64) {
    //     img.put_pixel(
    //         (s.0 + (t * (e.0 - s.0) as f64) as isize)
    //             .try_into()
    //             .unwrap(),
    //         (s.1 + (t * (e.1 - s.1) as f64) as isize)
    //             .try_into()
    //             .unwrap(),
    //         color,
    //     );
    //     t += 0.01;
    // }
}
