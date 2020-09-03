use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use std::ops::Add;
use std::cmp::PartialEq;
use super::complex::{Complex, Cartesian, Polar};
use super::real::{Real, real};
use super::im::{Im, im};
use super::exp::exp;



#[test]
fn addition_when_real_positive_and_im_positive() {
    let r = real(1.0);
    let i = im(1.0);

    assert_eq!((r + i).real(), 1.0);
    assert_eq!((r + i).im(), 1.0);
}


#[test]
fn additions() {
    let c = real(3.33) + im(5.55) + Cartesian{r: 1.0, i: 1.0} + Polar{argument: FRAC_PI_2, module: 1.0};

    assert_eq!(c.real(), 4.33);
    assert_eq!(c.im(), 7.55);
}

#[test]
fn multiplication_when_real_positive_and_im_positive() {
    let p = real(3.33) * im(1.0);

    relative_eq!(p.im(), 3.33);
    relative_eq!(p.real(), 0.00);
}

#[test]
fn multiplication_when_real_negative_and_im_positive() {
    let p = real(-3.33) * im(1.0);

    relative_eq!(p.im(), -3.33);
    relative_eq!(p.real(), 0.00);
}

#[test]
fn multiplications() {
    let c = (real(3.33) * im(5.55)) * Polar{argument: 1.0, module: 2.0} * Cartesian{r: 2.0, i: 2.0};

    assert_eq!(c.argument(), 1.0 + FRAC_PI_2 + FRAC_PI_4);
}

//#[test]
//fn when_exp() {
//    let i = im(FRAC_PI_2);
//    let r = exp(i);
//
//    relative_eq!((real(1.0) * r).real(), 0.0);
//}

