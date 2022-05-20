use crate::complex::Complex;
use crate::Coordinate;
use crate::Dimension;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Settings {
	pub host: String,
	pub port: u16,
	pub fractal: Fractal,
	pub image: Image,
	pub style: Style,
	pub dimension: Dimension,
	pub offset: Coordinate,
	pub connections: usize,
	pub timeout: u64,
}

#[derive(Debug, Deserialize)]
pub struct Image {
	pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Fractal {
	pub initial_value: Complex,
	pub iterations: u32,
	pub active_threshold: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
	Mandelbrot,
	Julia,
	Image,
}

impl Settings {
	pub fn new() -> anyhow::Result<Self> {
		let mut file = File::open("config.toml")?;
		let mut text = String::new();
		file.read_to_string(&mut text)?;

		Ok(toml::from_str(&text)?)
	}
}
