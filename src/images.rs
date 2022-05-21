use crate::coordinate::{Coordinate, Dimension};
use crate::pixel::{Color, Field, Pixel};
use image::imageops::FilterType;
use std::path::PathBuf;

pub fn image_to_field(dimension: Dimension, offset: Coordinate, filename: &str) -> Field {
	let path = PathBuf::from(filename);
	let image = image::open(&path).expect("Failed to load image.");

	let resized = image.resize(dimension.width as u32, dimension.height as u32, FilterType::Gaussian);
	let rgb = resized.to_rgb8();

	let mut field = Field::new(dimension);
	for x in 0..dimension.width {
		for y in 0..dimension.height {
			let pixel = rgb.get_pixel(x as u32, y as u32);
			field[x][y] = Pixel {
				coordinate: Coordinate {
					x: x + offset.x,
					y: y + offset.y,
				},
				color: Color::rgb(pixel[0], pixel[1], pixel[2]),
				active: true,
			}
		}
	}
	field
}
