use crate::coordinate::Coordinate;
use crate::coordinate::Dimension;
use image::Rgba;
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
use std::ops::Index;
use std::ops::IndexMut;
use std::option::Option;
use std::u32;

#[derive(Copy, Clone)]
pub struct Color(Rgba<u8>);

impl From<Rgba<u8>> for Color {
	fn from(rgba: Rgba<u8>) -> Self {
		Self(rgba)
	}
}

impl Color {
	pub fn red(&self) -> u8 {
		self.0[0]
	}

	pub fn green(&self) -> u8 {
		self.0[1]
	}

	pub fn blue(&self) -> u8 {
		self.0[2]
	}

	pub fn alpha(&self) -> u8 {
		self.0[3]
	}

	pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
		Rgba::from([red, green, blue, u8::MAX]).into()
	}

	pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Rgba::from([red, green, blue, alpha]).into()
	}

	pub fn null() -> Color {
		Self::rgba(0, 0, 0, 0)
	}

	#[allow(unused)]
	pub fn gray(color: u8) -> Color {
		Self::rgb(color, color, color)
	}

	pub fn gradient24(value: f64) -> Color {
		let value32 = (value * u32::MAX as f64) as u32;
		let blue = (value32 & 0xff) as u8;
		let green = ((value32 >> 8) & 0xff) as u8;
		let red = ((value32 >> 16) & 0xff) as u8;

		Self::rgb(red, green, blue)
	}

	#[allow(unused)]
	pub fn gray_gradient(value: f64) -> Color {
		let color = (255.0 * value) as u8;
		Self::gray(color)
	}
}

impl Display for Color {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
		match self.alpha() {
			u8::MAX => write!(formatter, "{:02x}{:02x}{:02x}", self.red(), self.green(), self.blue()),
			alpha => write!(
				formatter,
				"{:02x}{:02x}{:02x}{:02x}",
				self.red(),
				self.green(),
				self.blue(),
				alpha
			),
		}
	}
}

#[derive(Copy, Clone)]
pub struct Pixel {
	pub coordinate: Coordinate,
	pub color: Color,
}

impl Pixel {
	pub fn null() -> Pixel {
		Pixel {
			coordinate: Coordinate::null(),
			color: Color::null(),
		}
	}
}

impl Display for Pixel {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
		if self.color.alpha() == 0 {
			return Ok(());
		}

		writeln!(formatter, "PX {} {}\n", self.coordinate, self.color)
	}
}

pub struct Field {
	field: Vec<Vec<Pixel>>,
	dimension: Dimension,
}

impl Field {
	pub fn new(dimension: Dimension) -> Field {
		Field {
			field: vec![vec![Pixel::null(); dimension.height]; dimension.width],
			dimension,
		}
	}

	pub fn serialise(&self) -> Vec<Pixel> {
		let pixels = self.dimension.pixels();
		let mut serialised = vec![];
		serialised.reserve(pixels);
		for index in 0..pixels {
			let x = index % self.dimension.width;
			let y = index / self.dimension.width;

			serialised.push(self.field[x][y]);
		}

		let mut rng = rand::thread_rng();
		serialised.shuffle(&mut rng);

		serialised
	}

	pub fn coordinates_iterator(&self) -> FieldCoordinatesIterator {
		FieldCoordinatesIterator::new(self)
	}
}

impl Index<usize> for Field {
	type Output = Vec<Pixel>;
	fn index(&self, index: usize) -> &Vec<Pixel> {
		&self.field[index]
	}
}

impl IndexMut<usize> for Field {
	fn index_mut(&mut self, index: usize) -> &mut Vec<Pixel> {
		&mut self.field[index]
	}
}

pub struct FieldCoordinatesIterator {
	dimension: Dimension,
	index: usize,
}

/// Iterator for iteraing over `(x, y)` coordinate tuples of Pixels of a `Field`
impl FieldCoordinatesIterator {
	pub fn new(field: &Field) -> FieldCoordinatesIterator {
		FieldCoordinatesIterator {
			dimension: field.dimension,
			index: 0,
		}
	}
}

impl Iterator for FieldCoordinatesIterator {
	type Item = (usize, usize);

	fn next(&mut self) -> Option<Self::Item> {
		if self.index == self.dimension.pixels() {
			return None;
		}

		let x = self.index % self.dimension.width;
		let y = self.index / self.dimension.width;
		self.index += 1;

		Some((x, y))
	}
}
