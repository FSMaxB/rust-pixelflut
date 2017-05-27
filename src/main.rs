extern crate rand;
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
use pixel::Field;
use coordinate::Coordinate;
use coordinate::Dimension;

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

    let mut field = Field::new(DIMENSION);

    for (x, y) in field.coordinates_iterator() {
        let c = Complex {
            real: (x as f64 / DIMENSION.width as f64) * 3.0 - 2.0,
            imag: (y as f64 / DIMENSION.height as f64) * 2.5 - 1.25
        };

        let color = (255.0 * mandelbrot(c, ITERATIONS)) as u8;
        let active;
        if color < 40 {
            active = false;
        } else {
            active = true;
        }

        field[x][y] = Pixel {coordinate: Coordinate {x: x + OFFSET.x, y: y + OFFSET.y}, color: Color::gray(color), active: active};
    }

    let serialised_buffer = field.serialise();

    let mut command_buffer = (&serialised_buffer[1]).to_string();
    {
        for pixel in &serialised_buffer {
            command_buffer += &(pixel.to_string());
        }
    }

    let commands = command_buffer.as_bytes();

    loop {
        let _ = tcp_stream.write(&commands);
    }
}
