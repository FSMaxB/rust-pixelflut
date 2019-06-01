use crate::complex::Complex;
use crate::complex::abs;

pub fn mandelbrot(c: Complex, iterations: u32) -> f64 {
    let mut z = Complex { real: 0.0, imag: 0.0 };
    for i in 0..iterations {
        if abs(z) > 2.0 {
            return i as f64 / iterations as f64;
        }
        z = z * z + c;
    }

    return 1.0;
}

pub fn julia(z: Complex, initial_value: Complex, iterations: u32) -> f64 {
    let mut z = z;
    let c = initial_value;
    for i in 0..iterations {
        if abs(z) > 4.0 {
            return i as f64 / iterations as f64;
        }

        z = z * z + c;
    }

    return 1.0;
}
