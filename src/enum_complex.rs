use super::complex::Complex;
use std::ops::{Add, Deref, Mul};

#[derive(Debug, PartialEq)]
enum EComplex {
    Real(f32),
    Im(f32),
    Cartesian(f32, f32),
    Polar(f32, f32)
}

impl Add for EComplex {
    type Output = EComplex;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Cartesian(self.real() + rhs.real(), self.im() + rhs.im())
    }
}

impl Mul for EComplex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Real(0.0)
    }
}

impl Complex for EComplex {
    fn real(&self) -> f32 {
        match self {
            Self::Real(r) => *r,
            Self::Im(_) => 0.0,
            Self::Cartesian(r, _) => *r,
            Self::Polar(arg, module) => module * arg.cos()
        }
    }

    fn im(&self) -> f32 {
        0.0
    }

    fn argument(&self) -> f32 {
        0.0
    }

    fn module(&self) -> f32 {
        0.0
    }
}



fn real(r: f32) -> EComplex {
    EComplex::Real(r)
}

#[test]
fn create_real() {
    let r = real(3.33);

    assert_eq!(r, EComplex::Real(3.33));
}

#[test]
fn add() {
    let real_r = 3.33;
    let real_s = 5.55;

    let r = real(real_r);
    let s = real(real_s);

    assert_eq!((r + s).real(), real_r + real_s);
}
