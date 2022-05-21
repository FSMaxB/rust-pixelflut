use crate::{Coordinate, Pixel};
use image::RgbaImage;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub trait FrameSerializer: Send {
	fn serialize<'frame>(&mut self, frame: &'frame RgbaImage) -> Box<dyn Iterator<Item = Pixel> + 'frame>;
	fn duplicate(&self) -> Box<dyn FrameSerializer>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RowSerializer;

impl FrameSerializer for RowSerializer {
	fn serialize<'frame>(&mut self, frame: &'frame RgbaImage) -> Box<dyn Iterator<Item = Pixel> + 'frame> {
		Box::new(
			(0..frame.height())
				.into_iter()
				.flat_map(move |y| (0..frame.width()).into_iter().map(move |x| (x, y)))
				.map(move |(x, y)| Pixel {
					coordinate: Coordinate::new(x, y),
					color: (*frame.get_pixel(x, y)).into(),
				}),
		)
	}

	fn duplicate(&self) -> Box<dyn FrameSerializer> {
		Box::new(*self)
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ColumnSerializer;

impl FrameSerializer for ColumnSerializer {
	fn serialize<'frame>(&mut self, frame: &'frame RgbaImage) -> Box<dyn Iterator<Item = Pixel> + 'frame> {
		Box::new(
			(0..frame.width())
				.into_iter()
				.flat_map(move |x| (0..frame.height()).into_iter().map(move |y| (x, y)))
				.map(move |(x, y)| Pixel {
					coordinate: Coordinate::new(x, y),
					color: (*frame.get_pixel(x, y)).into(),
				}),
		)
	}

	fn duplicate(&self) -> Box<dyn FrameSerializer> {
		Box::new(*self)
	}
}

#[derive(Debug)]
pub struct RandomSerializer(SmallRng);

impl Default for RandomSerializer {
	fn default() -> Self {
		Self(SmallRng::from_entropy())
	}
}

impl Clone for RandomSerializer {
	fn clone(&self) -> Self {
		Default::default()
	}
}

impl FrameSerializer for RandomSerializer {
	fn serialize<'frame>(&mut self, frame: &'frame RgbaImage) -> Box<dyn Iterator<Item = Pixel> + 'frame> {
		let mut all_positions = (0..frame.width())
			.into_iter()
			.flat_map(move |x| (0..frame.height()).map(move |y| (x, y)))
			.collect::<Vec<_>>();
		all_positions.shuffle(&mut self.0);

		Box::new(all_positions.into_iter().map(move |(x, y)| Pixel {
			coordinate: Coordinate::new(x, y),
			color: (*frame.get_pixel(x, y)).into(),
		}))
	}

	fn duplicate(&self) -> Box<dyn FrameSerializer> {
		Box::new(self.clone())
	}
}
