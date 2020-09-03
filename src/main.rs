extern crate rust_complex;

use rust_complex::real::real;
use rust_complex::im::im;
use rust_complex::complex::{Cartesian, Complex, Polar};


fn main() {
    let mut p : Polar = Cartesian{r: 3.0, i: 0.0}.into();
    let mut center : Polar = Cartesian{r: 7.0, i: 0.0}.into();

    for _ in 0..1000 {
        p = (p * Polar{argument: 0.1, module: 1.0}).into();
        center = (center * Polar{argument: 0.03, module: 1.0}).into();

        let r = p + center;

     //   println!("{} {}", r.real(), r.im());
    }
}
