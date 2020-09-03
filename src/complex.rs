use std::f32::consts::{FRAC_PI_4, FRAC_PI_2, PI};
use std::ops::{Add, Mul};
use std::convert::From;

//pub trait Complex<T: Complex<T> = Self> : Add<T> + PartialEq + std::marker::Sized {
pub trait Complex<T> : Add<T> + Mul<T> + PartialEq + std::marker::Sized {
    fn real(&self) -> f32;
    fn im(&self) -> f32;
    fn argument(&self) -> f32;
    fn module(&self) -> f32;

    fn eq() -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cartesian {
    pub r: f32,
    pub i: f32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Polar {
    pub argument: f32,
    pub module: f32
}

impl Add for Cartesian {
    type Output = Self;

    fn add(self, c: Self) -> Self::Output {
        Cartesian{r: self.r + c.r, i: self.i + c.i}
    }
}

impl Mul for Cartesian {
    type Output = Polar;

    fn mul(self, c: Self) -> Polar {
        let self_polar : Polar = self.into();
        let other_polar : Polar = c.into();

        self_polar * other_polar
    }
}

impl Complex<Self> for Cartesian {
    fn real(&self) -> f32 {
        self.r
    }

    fn im(&self) -> f32 {
        self.i
    }

    fn argument(&self) -> f32 {
        let atan = (self.i / self.r).atan();

        match self.r.is_sign_positive() {
            false => PI + atan,
            true => atan,
        }

    }

    fn module(&self) -> f32 {
        (self.r.powf(2.0) + self.i.powf(2.0)).sqrt()
    }

    fn eq() -> bool {
        false
    }
}

impl Add for Polar {
    type Output = Cartesian;

    fn add(self, c: Self) -> Self::Output {
        let self_cartesian = Cartesian{r: self.real(), i: self.im()};
        let other_cartesian = Cartesian{r: c.real(), i: c.im()};

        self_cartesian + other_cartesian
    }
}

impl Mul for Polar {
    type Output = Self;

    fn mul(self, p: Polar) -> Self::Output {
        Polar{argument: self.argument + p.argument, module: self.module * p.module}
    }
}

impl Add<Cartesian> for Polar {
    type Output = Cartesian;

    fn add(self, c: Cartesian) -> Self::Output {
        let self_cartesian: Cartesian = self.into();

        self_cartesian + c
    }
}

impl Add<Polar> for Cartesian {
    type Output = Cartesian;

    fn add(self, p: Polar) -> Self::Output {
        let other_cartesian: Cartesian = p.into();

        self + other_cartesian
    }
}

impl Mul<Polar> for Cartesian {
    type Output = Polar;

    fn mul(self, p: Polar) -> Self::Output {
        let self_polar : Polar = self.into();

        self_polar * p
    }
}

impl Mul<Cartesian> for Polar {
    type Output = Self;

    fn mul(self, c: Cartesian) -> Self {
        let c_polar : Polar = c.into();

        self * c_polar
    }
}

impl Complex<Self> for Polar {
    fn real(&self) -> f32 {
        self.module * self.argument.cos()
    }

    fn im(&self) -> f32 {
        self.module * self.argument.sin()
    }

    fn argument(&self) -> f32 {
        self.argument % (2.0 * PI)
    }

    fn module(&self) -> f32 {
        self.module
    }

    fn eq() -> bool {
        false
    }
}

impl From<Cartesian> for Polar {
    fn from(c: Cartesian) -> Self {
        let argument = c.argument();
        let module = c.module();

        Polar{argument: argument, module: module}
    }
}

impl From<Polar> for Cartesian {
    fn from(p: Polar) -> Self {
        let real = p.real();
        let im = p.im();

        Cartesian{r: real, i: im}
    }
}

#[test]
fn cartesian() {
    let real_part = 3.33;
    let im_part = 5.55;

    let c = Cartesian{r: real_part, i: im_part};

    assert_eq!(c.real(), real_part);
    assert_eq!(c.im(), im_part);
}

#[test]
fn polar() {
    let angle = 3.33;
    let module = 5.55;

    let c = Polar{argument: angle, module: module};

    assert_eq!(c.argument(), angle);
    assert_eq!(c.module(), module);
}

#[test]
fn cartesian_argument_when_real_positive() {
    let c = Cartesian{r: 3.33, i: 0.0};

    assert_eq!(c.argument(), 0.0);
}

#[test]
fn cartesian_argument_when_real_negative() {
    let c = Cartesian{r: -3.33, i: 0.0};

    assert_eq!(c.argument(), PI);
}

#[test]
fn cartesian_argument_when_im_positive() {
    let c = Cartesian{r: 0.0, i: 3.33};

    assert_eq!(c.argument(), FRAC_PI_2);
}

#[test]
fn cartesian_argument_when_im_negative() {
    let c = Cartesian{r: 0.0, i: -3.33};

    assert_eq!(c.argument(), -FRAC_PI_2);
}

#[test]
fn cartesian_argument_when_positive_negative_and_im_positive() {
    let c = Cartesian{r: -1.0, i: 1.0};

    assert_eq!(c.argument(), PI - FRAC_PI_4);
}
#[test]
fn cartesian_module_when_real_positive_and_im_positive() {
    let c = Cartesian{r: 1.0, i: 1.0};

    assert_eq!(c.module(), 2.0f32.sqrt());
}

#[test]
fn cartesian_module_when_real_negative_and_im_positive() {
    let c = Cartesian{r: -1.0, i: 1.0};

    assert_eq!(c.module(), 2.0f32.sqrt());
}

#[test]
fn cartesian_module_when_real_negative_and_im_negative() {
    let c = Cartesian{r: -1.0, i: -1.0};

    assert_eq!(c.module(), 2.0f32.sqrt());
}

#[test]
fn cartesian_module_when_real_positive_and_im_negative() {
    let c = Cartesian{r: 1.0, i: -1.0};

    assert_eq!(c.module(), 2.0f32.sqrt());
}

#[test]
fn cartesian_to_polar_when_real_positive_and_im_positive() {
    let real_part = 3.33;
    let im_part = 5.55;

    let c = Cartesian{r: real_part, i: im_part};

    let p: Polar = c.into();

    relative_eq!(p.real(), real_part);
    relative_eq!(p.im(), im_part);
}

#[test]
fn cartesian_to_polar_when_real_negative_and_im_positive() {
    let real_part = -3.33;
    let im_part = 5.55;

    let c = Cartesian{r: real_part, i: im_part};

    let p: Polar = c.into();

    relative_eq!(p.real(), real_part);
    relative_eq!(p.im(), im_part);
}

#[test]
fn cartesian_to_polar_when_real_negative_and_im_negative() {
    let real_part = -3.33;
    let im_part = -5.55;

    let c = Cartesian{r: real_part, i: im_part};

    let p: Polar = c.into();

    relative_eq!(p.real(), real_part);
    relative_eq!(p.im(), im_part);
}

#[test]
fn cartesian_to_polar_when_real_positive_and_im_negative() {
    let real_part = 3.33;
    let im_part = -5.55;

    let c = Cartesian{r: real_part, i: im_part};

    let p: Polar = c.into();

    relative_eq!(p.real(), real_part);
    relative_eq!(p.im(), im_part);
}

#[test]
fn polar_to_cartesian_when_real_positive_and_im_positive() {
    let argument = FRAC_PI_4;
    let module = 3.33;

    let p = Polar{argument: argument, module: module};
    let c: Cartesian = p.into();

    relative_eq!(c.real(), module * argument.cos());
    relative_eq!(c.im(), module * argument.sin());
}

#[test]
fn polar_to_cartesian_when_real_negative_and_im_positive() {
    let argument = FRAC_PI_2 + FRAC_PI_4;
    let module = 3.33;

    let p = Polar{argument: argument, module: module};
    let c: Cartesian = p.into();

    relative_eq!(c.real(), -module * argument.cos());
    relative_eq!(c.im(), module * argument.sin());
}

#[test]
fn polar_to_cartesian_when_real_negative_and_im_negative() {
    let argument = PI + FRAC_PI_4;
    let module = 3.33;

    let p = Polar{argument: argument, module: module};
    let c: Cartesian = p.into();

    relative_eq!(c.real(), -module * argument.cos());
    relative_eq!(c.im(), - module * argument.sin());
}

#[test]
fn polar_to_cartesian_when_real_positive_and_im_negative() {
    let argument = PI + FRAC_PI_2 + FRAC_PI_4;
    let module = 3.33;

    let p = Polar{argument: argument, module: module};
    let c: Cartesian = p.into();

    relative_eq!(c.real(), module * argument.cos());
    relative_eq!(c.im(), - module * argument.sin());
}

#[test]
fn add_cartesian_to_polar() {
    let p = Polar{argument: 0.0, module: 1.0} + Cartesian{r: 1.0, i: 0.0};

    assert_eq!(p.real(), 2.0);
    assert_eq!(p.im(), 0.0);
}

#[test]
fn add_polar_to_cartesian() {
    let c = Cartesian{r: 1.0, i: 0.0} + Polar{argument: 0.0, module: 1.0};

    assert_eq!(c.real(), 2.0);
    assert_eq!(c.im(), 0.0);
}

#[test]
fn mul_cartesian_with_cartesian() {
    let c1 = Cartesian{r: 1.0, i: 0.0};
    let c2 = Cartesian{r: 0.0, i: 1.0};

    assert_eq!((c1 * c2).argument(), FRAC_PI_2);
    assert_eq!((c1 * c2).module(), 1.0);
}

#[test]
fn mul_polar_with_polar() {
    let p1 = Polar{argument: 3.33, module: 1.1};
    let p2 = Polar{argument: 5.55, module: 2.2};

    assert_eq!((p1 * p2).argument(), (3.33 + 5.55) % (PI + PI));
    assert_eq!((p1 * p2).module(), 1.1 * 2.2);
}

#[test]
fn mul_polar_with_polar_when_result_argument_larger_than_two_pi() {
    let p1 = Polar{argument: PI + FRAC_PI_4, module: 1.1};
    let p2 = Polar{argument: PI + FRAC_PI_4, module: 2.2};

    relative_eq!((p1 * p2).argument(), FRAC_PI_2);
    assert_eq!((p1 * p2).module(), 1.1 * 2.2);
}

#[test]
fn mul_cartesian_with_polar_when_real_positive() {
    let c = Cartesian{r: 1.0, i: 0.0};
    let p = Polar{argument: FRAC_PI_2, module: 1.0};

    assert_eq!((c * p).argument(), FRAC_PI_2);
    assert_eq!((c * p).module(), 1.0);
}

#[test]
fn mul_cartesian_with_polar_when_real_negative() {
    let c = Cartesian{r: -1.0, i: 0.0};
    let p = Polar{argument: FRAC_PI_2, module: 1.0};

    assert_eq!((c * p).argument(), c.argument() + FRAC_PI_2);
    assert_eq!((c * p).module(), 1.0);
}

#[test]
fn mul_polar_with_cartesian() {
    let c = Cartesian{r: 1.0, i: 0.0};
    let p = Polar{argument: FRAC_PI_2, module: 1.0};

    assert_eq!((p * c).argument(), FRAC_PI_2);
    assert_eq!((p * c).module(), 1.0);
}
