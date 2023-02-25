mod test {
    use crate::math::Vec2i;
    use crate::render::line::line;
    use crate::render::triangle::triangle;
    use image::{ImageOutputFormat, Rgb, RgbImage};
    use std::fs::OpenOptions;

    #[test]
    fn test_triangle() {
        let t0 = vec![Vec2i::new(10, 70), Vec2i::new(50, 160), Vec2i::new(70, 80)];
        let t1 = vec![Vec2i::new(180, 50), Vec2i::new(150, 1), Vec2i::new(70, 180)];
        let t2 = vec![
            Vec2i::new(180, 150),
            Vec2i::new(120, 160),
            Vec2i::new(130, 180),
        ];

        let width = 800;
        let height = 800;
        // println!("{:?}", f);

        let mut img = RgbImage::new(width, height);
        let red = Rgb([255, 0, 0]);
        let white = Rgb([255, 255, 255]);
        let green = Rgb([0, 255, 0]);
        triangle(t0[0], t0[1], t0[2], &mut img, red);
        triangle(t1[0], t1[1], t1[2], &mut img, white);
        triangle(t2[0], t2[1], t2[2], &mut img, green);

        let mut output = OpenOptions::new()
            .write(true)
            .create(true)
            .open("test_triangle.tga")
            .unwrap();
        img.write_to(&mut output, ImageOutputFormat::Tga).unwrap();
    }
}
