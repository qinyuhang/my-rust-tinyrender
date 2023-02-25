pub struct Matrix2 {}
pub struct Matrix3 {}
pub struct Matrix4 {
    pub data: [f64; 16],
}
// impl Index<usize> for Matrix4 {
//     type Output = Vec4;
//     fn index(&self, idx: usize) -> &Self::Output {
//         let x = Vec4::new(0.0, 0.0, 0.0, 1.0);
//         &x
//         // &self.data[idx]
//     }
// }
