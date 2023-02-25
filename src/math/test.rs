mod test {
    #[allow(unused)]
    use crate::math::*;
    #[test]
    fn test_make_v3() {
        #[allow(unused)]
        let v = Vec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        // let v2 = Vec4::new(1, 2, 3, 1);

        let v3 = v.clone() + v;

        assert_eq!(v3.x, 2.0);
    }
}
