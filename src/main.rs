extern crate rand;
use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::thread;

mod complex;
mod fractal;
mod pixel;
mod coordinate;
mod settings;
mod images;

use complex::Complex;
use fractal::mandelbrot;
use fractal::julia;
use pixel::Pixel;
use pixel::Color;
use pixel::Field;
use coordinate::Coordinate;
use coordinate::Dimension;
use crate::settings::{Settings, Style};
use crate::settings::Style::Mandelbrot;
use std::time::Duration;
use crate::images::image_to_field;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use core::borrow::{Borrow, BorrowMut};
use std::sync::atomic::Ordering;

fn main() {
    let settings = Settings::new()
        .map_err(|error| eprintln!("Failed to read config with error: {}", error))
        .unwrap();

    let field = match settings.style {
        Style::Julia | Style::Mandelbrot => {
            let mut field = Field::new(settings.dimension);

            for (x, y) in field.coordinates_iterator() {
                let fractal_width = 4.0;
                let fractal_height = (settings.dimension.height as f64 / settings.dimension.width as f64) * fractal_width;
                let fractal_x_offset = 0.0;
                let fractal_y_offset = 0.0;
                let c = Complex {
                    real: (x as f64 / settings.dimension.width as f64) * fractal_width - fractal_width/2.0 + fractal_x_offset,
                    imag: (y as f64 / settings.dimension.height as f64) * fractal_height - fractal_height/2.0 + fractal_y_offset,
                };

                let iteration_factor = match settings.style {
                    Style::Julia => julia(c, settings.fractal.initial_value, settings.fractal.iterations),
                    Style::Mandelbrot => mandelbrot(c, settings.fractal.iterations),
                    _ => panic!("Not a fractal!"),
                };
                let active= if iteration_factor < settings.fractal.active_threshold {
                    false
                } else {
                    true
                };
                let color = Color::gradient24(iteration_factor);

                field[x][y] = Pixel {coordinate: Coordinate {x: x + settings.offset.x, y: y + settings.offset.y}, color, active};
            }
            field
        },
        Style::Image => image_to_field(settings.dimension, settings.offset, &settings.image.path),
    };

    let serialised_buffer = field.serialise();

    let mut connections = vec![];

    let host_and_port = format!("{}:{}", settings.host, settings.port);
    for i in 0..settings.connections {
        let tcp_result = TcpStream::connect(&host_and_port);
        if !tcp_result.is_ok() {
            println!("Failed to open TCP stream {}.", i);
        } else {
            println!("Opened TCP stream {}.", i);
        }

        connections.push(tcp_result.unwrap());
    }

    let divisor = serialised_buffer.len() / settings.connections;
    let mut connection_slices = vec![];
    for i in 0..settings.connections {
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

    let mut threads = vec![];

    let keep_running = Arc::new(AtomicBool::new(true));
    {
        let keep_running_copy = keep_running.clone();
        ctrlc::set_handler(move || {
            eprintln!("Ctrl-C pressed");
            keep_running_copy.store(false, Ordering::SeqCst);
        }).expect("Failed to set Ctrl-C handler.");
    }

    for connection_number in 0..settings.connections {
        let mut connection = connections.pop().unwrap();
        connection.set_write_timeout(Some(Duration::from_secs(10)));
        let command = connection_commands.pop().unwrap();

        let keep_running = keep_running.clone();
        threads.push(thread::spawn(move || {
            'retry: for current_try in 1..4 {
                loop {
                    let result = connection.write(&(command.as_bytes()))
                        .map_err(|error| eprintln!("Error: {}", error));
                    if result.is_err() {
                        eprintln!("Failed writing on connection {}, aborting.", connection_number);
                        break;
                    }
                    if !keep_running.load(Ordering::SeqCst) {
                        eprintln!("Aborted. Closing connection.");
                        break 'retry;
                    }
                }
                thread::sleep(Duration::from_secs(1));
            }
            connection.shutdown(Shutdown::Both);
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
}
