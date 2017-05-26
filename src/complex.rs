use std::f64;
use std::ops::Add;
use std::ops::Mul;

#[derive(Copy,Clone)]
pub struct Complex {
    pub real: f64,
    pub imag: f64
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

pub fn abs(number: Complex) -> f64 {
    return f64::sqrt(number.real * number.real + number.imag * number.imag);
}
