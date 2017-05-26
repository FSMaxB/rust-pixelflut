extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample,Range};
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

fn pixel(point: &Point) -> String {
    return format!("PX {} {} {:02x}{:02x}{:02x}", point.x, point.y, point.red, point.green, point.blue);
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
            return i as f64 / iterations as f64;
        }
        z = z * z + c;
    }

    return 1.0;
}

#[derive(Copy,Clone)]
struct Point {
    x: usize,
    y: usize,
    red: u8,
    green: u8,
    blue: u8
}

fn write_vector_to_stream(points: &Vec<Point>, stream: &mut TcpStream) -> bool {
    let mut success = true;

    for point in points {
        success |= write_to_stream(&pixel(point), stream);
    }

    return success;
}

fn main() {
    let tcp_option = TcpStream::connect("94.45.231.39:1234");
    if !tcp_option.is_ok() {
        println!("Failed to open TCP stream.");
        exit(1);
    }
    let mut tcp_stream = tcp_option.unwrap();
    const ITERATIONS : u8 = 30;
    const WIDTH : usize = 1000;
    const Y_OFFSET : usize = 100;
    const X_OFFSET : usize = 100;
    const HEIGHT : usize = (2 * WIDTH) / 3;

    let mut buffer : Vec<Vec<f64>> = vec![vec![0.0; HEIGHT]; WIDTH];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let c = Complex {
                real: (x as f64 / WIDTH as f64) * 3.0 - 2.0,
                imag: (y as f64 / HEIGHT as f64) * 2.5 - 1.25
            };

            buffer[x][y] = mandelbrot(c, ITERATIONS);
        }
    }

    let x_range = Range::new(0, WIDTH);
    let y_range = Range::new(0, HEIGHT);
    let mut rng = rand::thread_rng();

    const NULL_POINT : Point = Point {x: 0, y: 0, red: 0, green: 0, blue: 0};
    let mut serialised_buffer : Vec<Point> = vec![NULL_POINT; HEIGHT * WIDTH];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let color = (255.0 * buffer[x][y]) as u8;
            let point = Point {x: x + X_OFFSET, y: y + Y_OFFSET, red: color, green: color, blue: color};
            let index = y * WIDTH + x;
            serialised_buffer[index] = point;
        }
    }

    rng.shuffle(&mut serialised_buffer[..]);

    loop {
        write_vector_to_stream(&serialised_buffer, &mut tcp_stream);
    }
}
