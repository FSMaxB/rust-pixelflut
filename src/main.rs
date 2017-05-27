extern crate rand;
use rand::Rng;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::exit;

mod complex;
mod fractal;
mod pixel;
mod coordinate;
use complex::Complex;
use fractal::mandelbrot;
use pixel::Pixel;
use pixel::Color;
use coordinate::Coordinate;
use coordinate::Dimension;

fn write_to_stream(line: &[u8], stream: &mut TcpStream) -> bool {
    let written;
    let result = stream.write(line);
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

fn write_vector_to_stream(pixels: &Vec<Pixel>, stream: &mut TcpStream) -> bool {
    let mut success = true;

    for pixel in pixels {
        success |= write_to_stream(pixel.to_string().as_bytes(), stream);
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
    const WIDTH : usize = 600;
    const DIMENSION : Dimension = Dimension {width: WIDTH, height: (2 * WIDTH) / 3};
    const OFFSET : Coordinate = Coordinate {x: 1020, y: 0};

    let mut buffer : Vec<Vec<f64>> = vec![vec![0.0; DIMENSION.height]; DIMENSION.width];

    for x in 0..DIMENSION.width {
        for y in 0..DIMENSION.height {
            let c = Complex {
                real: (x as f64 / DIMENSION.width as f64) * 3.0 - 2.0,
                imag: (y as f64 / DIMENSION.height as f64) * 2.5 - 1.25
            };

            buffer[x][y] = mandelbrot(c, ITERATIONS);
        }
    }

    let mut rng = rand::thread_rng();

    let mut serialised_buffer : Vec<Pixel> = vec![Pixel::null(); DIMENSION.pixels()];

    for x in 0..DIMENSION.width {
        for y in 0..DIMENSION.height {
            let color = (255.0 * buffer[x][y]) as u8;
            let active;
            if color < 40 {
                active = false;
            } else {
                active = true;
            }
            let pixel = Pixel {coordinate: Coordinate {x: x + OFFSET.x, y: y + OFFSET.y}, color: Color::gray(color), active: active};
            let index = y * DIMENSION.height + x;
            serialised_buffer[index] = pixel;
        }
    }

    rng.shuffle(&mut serialised_buffer[..]);

    let mut command_buffer = (&serialised_buffer[1]).to_string();

    {
        for pixel in &serialised_buffer {
            command_buffer += &(pixel.to_string());
        }
    }

    let commands = command_buffer.as_bytes();

    loop {
        write_to_stream(&commands, &mut tcp_stream);
    }
}
