extern crate rand;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

mod complex;
mod fractal;
mod pixel;
mod coordinate;
use complex::Complex;
use fractal::mandelbrot;
use fractal::julia;
use pixel::Pixel;
use pixel::Color;
use pixel::Field;
use coordinate::Coordinate;
use coordinate::Dimension;

fn main() {
    const ITERATIONS : u32 = 1000;
    const DIMENSION : Dimension = Dimension {width: 1920, height: 1200};
    const OFFSET : Coordinate = Coordinate {x: 0, y: 0};

    let host = std::env::args().nth(1).expect("Expected host:port as command line argument!");

    let mut field = Field::new(DIMENSION);

    for (x, y) in field.coordinates_iterator() {
        let fractal_width = 4.0;
        let fractal_height = (DIMENSION.height as f64 / DIMENSION.width as f64) * fractal_width;
        let fractal_x_offset = 0.0;
        let fractal_y_offset = 0.0;
        let c = Complex {
            real: (x as f64 / DIMENSION.width as f64) * fractal_width - fractal_width/2.0 + fractal_x_offset,
            imag: (y as f64 / DIMENSION.height as f64) * fractal_height - fractal_height/2.0 + fractal_y_offset,
        };

        const ACTIVE_THRESHOLD : f64 = 0.0;
        //let iteration_factor = mandelbrot(c, ITERATIONS);
        let iteration_factor = julia(c, ITERATIONS);
        let active;
        if iteration_factor < ACTIVE_THRESHOLD {
            active = false;
        } else {
            active = true;
        }
        let color = Color::gradient24(iteration_factor);

        field[x][y] = Pixel {coordinate: Coordinate {x: x + OFFSET.x, y: y + OFFSET.y}, color: color, active: active};
    }

    let serialised_buffer = field.serialise();

    const CONNECTIONS : usize = 1;

    let mut connections = vec![];

    for i in 0..CONNECTIONS {
        let tcp_result = TcpStream::connect(&host);
        if !tcp_result.is_ok() {
            println!("Failed to open TCP stream {}.", i);
        } else {
            println!("Opened TCP stream {}.", i);
        }

        connections.push(tcp_result.unwrap());
    }

    let divisor = serialised_buffer.len() / CONNECTIONS;
    let mut connection_slices = vec![];
    for i in 0..CONNECTIONS {
        connection_slices.push(&serialised_buffer[(i*divisor)..((i+1)*divisor)]);
    }

    let mut connection_commands = vec![];
    for slice in connection_slices {
        let mut command = "".to_string();
        for pixel in slice {
            command += &(pixel.to_string());
        }
        connection_commands.push(command);
    }

    /*let mut command_buffer = (&serialised_buffer[1]).to_string();
    {
        for pixel in &serialised_buffer {
            command_buffer += &(pixel.to_string());
        }
    }*/

    let mut threads = vec![];

    for _ in 0..CONNECTIONS {
        let mut connection = connections.pop().unwrap();
        let command = connection_commands.pop().unwrap();

        threads.push(thread::spawn(move || {
            loop {
                let _ = connection.write(&(command.as_bytes()));
            }
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
}
