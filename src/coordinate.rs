use serde::Deserialize;
use std::string::ToString;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Coordinate {
	pub x: usize,
	pub y: usize,
}

impl ToString for Coordinate {
	fn to_string(&self) -> String {
		format!("{} {}", self.x, self.y)
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
