use super::complex::Complex;
use super::im::Im;
use super::rotor::Rotor;

pub fn exp(i: Im) -> Rotor {
    Rotor {angle: i.im()}
}
