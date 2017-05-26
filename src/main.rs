extern crate rand;
use rand::Rng;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::exit;

mod complex;
mod fractal;
mod pixel;
use complex::Complex;
use fractal::mandelbrot;
use pixel::Pixel;
use pixel::pixel_command;

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
        success |= write_to_stream(pixel_command(pixel).as_bytes(), stream);
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
    const X_OFFSET : usize = 1020;
    const Y_OFFSET : usize = 0;
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

    const NULL_PIXEL : Pixel = Pixel {x: 0, y: 0, red: 0, green: 0, blue: 0, active: false};
    let mut serialised_buffer : Vec<Pixel> = vec![NULL_PIXEL; HEIGHT * WIDTH];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let color = (255.0 * buffer[x][y]) as u8;
            let active;
            if color < 40 {
                active = false;
            } else {
                active = true;
            }
            let pixel = Pixel {x: x + X_OFFSET, y: y + Y_OFFSET, red: color, green: color, blue: color, active: active};
            let index = y * WIDTH + x;
            serialised_buffer[index] = pixel;
        }
    }

    rng.shuffle(&mut serialised_buffer[..]);

    let mut command_buffer = pixel_command(&serialised_buffer[1]);

    {
        for pixel in &serialised_buffer {
            command_buffer += &pixel_command(pixel);
        }
    }

    let commands = command_buffer.as_bytes();

    loop {
        write_to_stream(&commands, &mut tcp_stream);
    }
}
