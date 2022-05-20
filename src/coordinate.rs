use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Coordinate {
	pub x: usize,
	pub y: usize,
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
