mod line;
mod triangle;

use crate::math::{Vec2i, Vec3f};
use image::{ImageOutputFormat, Rgb, RgbImage};
use line::line;
use std::fs::OpenOptions;
use wavefront::Obj;

mod test;

pub fn do_render() {
    let f = Obj::from_file("./obj/african_head.obj").unwrap();
    let width = 800;
    let height = 800;
    // println!("{:?}", f);

    let mut img = RgbImage::new(width, height);
    println!("model has {} triangles", f.triangles().count());
    for z in f.triangles() {
        for i in 0..3 {
            // println!("line: {} {}", i, (i + 1) % 3);
            // println!("line: {:?} {:?}", z[i].position(), z[(i + 1) % 3]);
            let p0 = z[i].position();
            let p1 = z[(i + 1) % 3].position();
            let v0 = Vec3f::new(p0[0] as f64, p0[1] as f64, p0[2] as f64);
            let v1 = Vec3f::new(p1[0] as f64, p1[1] as f64, p1[2] as f64);
            let x0 = (v0.x + 1.0) * width as f64 / 2.1;
            let y0 = (v0.y + 1.0) * height as f64 / 2.1;
            let x1 = (v1.x + 1.0) * width as f64 / 2.1;
            let y1 = (v1.y + 1.0) * height as f64 / 2.1;
            line(
                Vec2i::new(x0 as isize, y0 as isize),
                Vec2i::new(x1 as isize, y1 as isize),
                &mut img,
                Rgb([255, 255, 255]),
            );
        }
    }
    // line((20, 13), (40, 80), &mut img, Rgb([255, 255, 255]));
    // line((80, 40), (13, 20), &mut img, Rgb([255, 255, 255]));
    // line((13, 20), (80, 40), &mut img, Rgb([255, 0, 0]));

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open("out.tga")
        .unwrap();
    img.write_to(&mut output, ImageOutputFormat::Tga).unwrap();
}
