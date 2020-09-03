use super::complex::Complex;
use super::real::Real;

#[derive(Debug)]
pub struct Rotor {
    pub angle: f32
}

impl PartialEq<Real> for Rotor {
    fn eq(&self, i: &Real) -> bool {
        self.angle == i.real()
    }
}

