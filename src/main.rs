use std::io::prelude::*;
use std::net::TcpStream;
use std::process::exit;
use std::ops::Add;
use std::ops::Mul;
use std::f64;

fn write_to_stream(line: &String, stream: &mut TcpStream) -> bool {
    let mut written;
    let mut result = stream.write(line.as_bytes());
    if !result.is_ok() {
        return false;
    } else {
        written = result.unwrap();
    }

    result = stream.write(b"\n");
    if !result.is_ok() {
        return false;
    } else {
        written += result.unwrap();
    }

    if written != (line.len() + 1) {
        return false;
    }

    return true;
}

fn pixel(x: u16, y: u16, red: u8, green: u8, blue: u8) -> String {
    return format!("PX {} {} {:02x}{:02x}{:02x}", x, y, red, green, blue);
}

#[derive(Copy,Clone)]
struct Complex {
    real: f64,
    imag: f64
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

fn abs(number: Complex) -> f64 {
    return f64::sqrt(number.real * number.real + number.imag * number.imag);
}

fn mandelbrot(c: Complex, iterations: u8) -> f64 {
    let mut z = Complex { real: 0.0, imag: 0.0 };
    for i in 0..iterations {
        if abs(z) > 4.0 {
            return (i as f64 / iterations as f64);
        }
        z = z * z + c;
    }

    return 1.0;
}

fn main() {
    let tcp_option = TcpStream::connect("94.45.234.7:1234");
    if !tcp_option.is_ok() {
        println!("Failed to open TCP stream.");
        exit(1);
    }
    let mut tcp_stream = tcp_option.unwrap();
    let iterations = 30;
    let width = 1000;
    let offset = 0;
    let height = (2 * width) / 3;

    loop {
        for x in 0..(width + 1) {
            for y in 0..(height + 1) {
                let c = Complex {
                    real: (x as f64 / width as f64) * 3.0 - 2.0,
                    imag: (y as f64 / height as f64) * 2.5 - 1.25
                };

                let factor = mandelbrot(c, iterations);
                let color = (255.0 * factor) as u8;
                write_to_stream(&pixel(x + offset, y + offset, color, color, color), &mut tcp_stream);
            }
        }
    }
}
