extern crate rand;
use rand::Rng;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::exit;

mod complex;
use complex::Complex;
use complex::abs;

mod pixel;
use pixel::Point;
use pixel::pixel_command;

fn write_to_stream(line: &[u8], stream: &mut TcpStream) -> bool {
    let mut written;
    let mut result = stream.write(line);
    if !result.is_ok() {
        return false;
    } else {
        written = result.unwrap();
    }

    if written != (line.len() + 1) {
        return false;
    }

    return true;
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

fn write_vector_to_stream(points: &Vec<Point>, stream: &mut TcpStream) -> bool {
    let mut success = true;

    for point in points {
        success |= write_to_stream(pixel_command(point).as_bytes(), stream);
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
    const WIDTH : usize = 400;
    const X_OFFSET : usize = 0;
    const Y_OFFSET : usize = 620;
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

    let mut rng = rand::thread_rng();

    const NULL_POINT : Point = Point {x: 0, y: 0, red: 0, green: 0, blue: 0, active: false};
    let mut serialised_buffer : Vec<Point> = vec![NULL_POINT; HEIGHT * WIDTH];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let color = (255.0 * buffer[x][y]) as u8;
            let active;
            if color < 40 {
                active = false;
            } else {
                active = true;
            }
            let point = Point {x: x + X_OFFSET, y: y + Y_OFFSET, red: color, green: color, blue: color, active: active};
            let index = y * WIDTH + x;
            serialised_buffer[index] = point;
        }
    }

    rng.shuffle(&mut serialised_buffer[..]);

    let mut command_buffer = pixel_command(&serialised_buffer[1]);

    {
        for point in &serialised_buffer {
            command_buffer += &pixel_command(point);
        }
    }

    let commands = command_buffer.as_bytes();

    loop {
        write_to_stream(&commands, &mut tcp_stream);
    }
}
