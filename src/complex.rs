use std::f64;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use serde_derive::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Complex {
    pub real: f64,
    pub imag: f64
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            imag: -self.imag
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        Complex {
            real: ((self.real * other.real) + (self.imag * other.imag)) / ((other.real * other.real) + (other.imag * other.imag)),
            imag: ((self.imag * other.real) - (self.real * other.imag)) / ((other.real * other.real) + (other.imag * other.imag))

        }
    }
}

pub fn abs(number: Complex) -> f64 {
    return f64::sqrt(number.real * number.real + number.imag * number.imag);
}
