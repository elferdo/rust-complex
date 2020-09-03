use std::ops::{Add, Mul};

use super::complex::{Cartesian, Polar, Complex};
use super::im::{Im};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Real  {
    r: f32
}

impl Add for Real {
    type Output = Self;

    fn add(self, r: Real) -> Self::Output {
        Real{r: self.r + r.r}
    }
}

impl Mul for Real {
    type Output = Self;

    fn mul(self, r: Real) -> Self::Output {
        Real{r: self.r * r.r}
    }
}

impl Mul<Im> for Real {
    type Output = Polar;

    fn mul(self, i: Im) -> Self::Output {
        Polar{argument: i.argument(), module: self.r * i.module()}
    }
}

impl Add<Im> for Real {
    type Output = Cartesian;

    fn add(self, x: Im) -> Self::Output {
        Cartesian{r: self.r, i: x.im()}
    }
}

impl Complex for Real {
    fn real(&self) -> f32 {
        self.r
    }

    fn im(&self) -> f32 {
        0.0
    }

    fn argument(&self) -> f32 {
        0.0
    }

    fn module(&self) -> f32 {
        self.r
    }
}

pub fn real(x: f32) -> Real {
    Real{r: x}
}

#[test]
fn when_real() {
    assert_eq!(real(1.0).real(), 1.0);
}

#[test]
fn complex_argument() {
    let r = real(3.33);

    assert_eq!(r.argument(), 0.0);
}

#[test]
fn complex_module() {
    let r = real(3.33);

    assert_eq!(r.module(), 3.33);
}

#[test]
fn addition() {
    let i1 = real(1.11);
    let i2 = real(2.22);

    assert_eq!((i1 + i2).real(), 1.11 + 2.22);
}

#[test]
fn multiplication() {
    let i1 = real(1.11);
    let i2 = real(2.22);

    assert_eq!((i1 * i2).argument(), 0.0);
    assert_eq!((i1 * i2).module(), 1.11 * 2.22);
}
