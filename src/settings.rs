use crate::complex::Complex;
use crate::Dimension;
use crate::Coordinate;
use serde_derive::Deserialize;
use config::{ConfigError, Config, File};

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
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Fractal {
    pub initial_value: Complex,
    pub iterations: u32,
    pub active_threshold:f64,
}

#[derive(Debug, Deserialize)]
pub enum Style {
    Mandelbrot,
    Julia,
    Image,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(File::with_name("config"))?;

        config.try_into()
    }
}