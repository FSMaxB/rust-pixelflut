use crate::coordinate::{Dimension, Coordinate};
use crate::pixel::{Field, Pixel, Color};
use image::jpeg::JPEGDecoder;
use image::GenericImage;
use image::DynamicImage;
use image::{ImageDecoder, FilterType};
use std::path::{Path, PathBuf};
use std::fs::File;

pub fn image_to_field(dimension: Dimension, offset: Coordinate, filename: &str) -> Field {
    let path = PathBuf::from(filename);
    let mut image = image::open(&path).expect("Failed to load image.");

    let resized = image.resize(dimension.width as u32, dimension.height as u32, FilterType::Gaussian);
    let rgb = resized.to_rgb();

    let mut field = Field::new(dimension);
    for x in 0..dimension.width {
        for y in 0..dimension.height {
            let pixel = rgb.get_pixel(x as u32, y as u32);
            field[x][y] = Pixel {
                coordinate: Coordinate {
                    x: x + offset.x,
                    y: y + offset.y,
                },
                color: Color {
                    red: pixel.data[0],
                    green: pixel.data[1],
                    blue: pixel.data[2],
                    alpha: None
                },
                active: true,
            }
        }
    }
    field
}