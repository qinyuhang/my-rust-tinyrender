#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub qi: f64,
    pub qj: f64,
    pub qk: f64,
    pub qw: f32,
}

// impl std::ops::Mul for Quaternion {
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self::Output {
//         Quaternion { qi: self.j }
//     }
// }

#[allow(unused)]
impl std::ops::Add for Quaternion {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self
    }
}
