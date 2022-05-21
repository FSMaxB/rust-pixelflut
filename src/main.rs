extern crate rand;
use std::net::ToSocketAddrs;
use std::thread;
use std::time::Duration;

mod complex;
mod coordinate;
mod fractal;
mod frame_painter;
mod frame_serializer;
mod images;
mod pixel;
mod pixel_backend;
mod settings;

use crate::frame_painter::io_uring::IoUringFramePainter;
use crate::frame_painter::FramePainter;
use crate::settings::{Settings, Style};
use anyhow::Context;
use coordinate::Coordinate;
use coordinate::Dimension;
use pixel::Pixel;

fn main() -> anyhow::Result<()> {
	let settings = Settings::new()
		.map_err(|error| eprintln!("Failed to read config with error: {}", error))
		.unwrap();

	let image = match settings.style {
		Style::Julia | Style::Mandelbrot => {
			/*
			let mut field = Field::new(settings.dimension);

			for (x, y) in field.coordinates_iterator() {
				let fractal_width = 4.0;
				let fractal_height =
					(settings.dimension.height as f64 / settings.dimension.width as f64) * fractal_width;
				let fractal_x_offset = 0.0;
				let fractal_y_offset = 0.0;
				let c = Complex {
					real: (x as f64 / settings.dimension.width as f64) * fractal_width - fractal_width / 2.0
						+ fractal_x_offset,
					imag: (y as f64 / settings.dimension.height as f64) * fractal_height - fractal_height / 2.0
						+ fractal_y_offset,
				};

				let iteration_factor = match settings.style {
					Style::Julia => julia(c, settings.fractal.initial_value, settings.fractal.iterations),
					Style::Mandelbrot => mandelbrot(c, settings.fractal.iterations),
					_ => panic!("Not a fractal!"),
				};
				let color = if iteration_factor < settings.fractal.active_threshold {
					Color::null()
				} else {
					Color::gradient24(iteration_factor)
				};

				field[x][y] = Pixel {
					coordinate: Coordinate {
						x: x + settings.offset.x,
						y: y + settings.offset.y,
					},
					color,
				};
			}
			field

			 */
			todo!()
		}
		Style::Image => image::open(settings.image.path).context("Failed to load image.")?,
	};

	let socket_address = format!("{}:{}", settings.host, settings.port)
		.to_socket_addrs()?
		.into_iter()
		.next()
		.unwrap();
	dbg!(socket_address);
	let mut frame_painter = IoUringFramePainter::start(socket_address, image);

	frame_painter.update_dimensions(settings.dimension);
	frame_painter.update_position(settings.offset);
	frame_painter.update_stream_count(settings.connections);

	loop {
		thread::sleep(Duration::from_secs(10));
	}
}
