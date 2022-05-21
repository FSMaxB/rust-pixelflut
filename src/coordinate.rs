use serde::Deserialize;
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub struct Coordinate {
	pub x: usize,
	pub y: usize,
}

impl Coordinate {
	pub fn new(x: u32, y: u32) -> Self {
		Self {
			x: x.try_into().unwrap(),
			y: y.try_into().unwrap(),
		}
	}
}

impl Add for Coordinate {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let Self { x, y } = self;
		let Self { x: rhs_x, y: rhs_y } = rhs;
		Self {
			x: x + rhs_x,
			y: y + rhs_y,
		}
	}
}

impl AddAssign for Coordinate {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Display for Coordinate {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
		write!(formatter, "{} {}", self.x, self.y)
	}
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Dimension {
	pub width: usize,
	pub height: usize,
}

impl Dimension {
	pub fn pixels(self) -> usize {
		self.width * self.height
	}
}

impl Coordinate {
	pub fn null() -> Coordinate {
		Coordinate { x: 0, y: 0 }
	}
}
