use std::f32::consts::{FRAC_PI_2, PI};
use std::ops::{Add, Mul};
use super::complex::{Cartesian, Polar, Complex};
use super::real::{Real};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Im {
    i: f32
}

impl Add for Im {
    type Output = Self;

    fn add(self, i: Im) -> Self::Output {
        Im{i: self.i + i.i}
    }
}

impl Mul for Im {
    type Output = Polar;

    fn mul(self, i: Im) -> Self::Output {
        Polar{argument: PI, module: self.i * i.i}
    }
}

impl Add<Real> for Im {
    type Output = Cartesian;

    fn add(self, x: Real) -> Self::Output {
        x + self
    }
}

impl Complex for Im {
    fn real(&self) -> f32 {
        0.0
    }

    fn im(&self) -> f32 {
        self.i
    }

    fn argument(&self) -> f32 {
        self.i.signum() * FRAC_PI_2
    }

    fn module(&self) -> f32 {
        self.i.abs()
    }
}

pub fn im(y: f32) -> Im {
    Im{i: y}
}

#[test]
fn when_im() {
    assert_eq!(im(1.0).im(), 1.0);
}

#[test]
fn complex_argument() {
    let r = im(3.33);

    assert_eq!(r.argument(), FRAC_PI_2);
}

#[test]
fn complex_module() {
    let r = im(3.33);

    assert_eq!(r.module(), 3.33);
}

#[test]
fn addition() {
    let i1 = im(1.11);
    let i2 = im(2.22);

    assert_eq!((i1 + i2).im(), 1.11 + 2.22);
}

#[test]
fn multiplication() {
    let i1 = im(1.11);
    let i2 = im(2.22);

    assert_eq!((i1 * i2).argument(), PI);
    assert_eq!((i1 * i2).module(), 1.11 * 2.22);
}
