use image::{Rgba, RgbaImage};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub trait FrameSerializer {
	fn serialize<'frame>(&mut self, frame: &'frame RgbaImage) -> Box<dyn Iterator<Item = Rgba<u8>> + 'frame>;
}

#[derive(Debug, Default)]
pub struct RowSerializer;

impl FrameSerializer for RowSerializer {
	fn serialize<'frame>(&mut self, frame: &'frame RgbaImage) -> Box<dyn Iterator<Item = Rgba<u8>> + 'frame> {
		Box::new(frame.rows().flatten().copied())
	}
}

#[derive(Debug, Default)]
pub struct ColumnSerializer;

impl FrameSerializer for ColumnSerializer {
	fn serialize<'frame>(&mut self, frame: &'frame RgbaImage) -> Box<dyn Iterator<Item = Rgba<u8>> + 'frame> {
		Box::new(
			(0..frame.width())
				.into_iter()
				.flat_map(move |x| (0..frame.height()).into_iter().map(move |y| (x, y)))
				.map(move |(x, y)| *frame.get_pixel(x, y)),
		)
	}
}

#[derive(Debug)]
pub struct RandomSerializer(SmallRng);

impl Default for RandomSerializer {
	fn default() -> Self {
		Self(SmallRng::from_entropy())
	}
}

impl FrameSerializer for RandomSerializer {
	fn serialize<'frame>(&mut self, frame: &'frame RgbaImage) -> Box<dyn Iterator<Item = Rgba<u8>> + 'frame> {
		let mut all_positions = (0..frame.width())
			.into_iter()
			.flat_map(move |x| (0..frame.height()).map(move |y| (x, y)))
			.collect::<Vec<_>>();
		all_positions.shuffle(&mut self.0);

		Box::new(all_positions.into_iter().map(move |(x, y)| *frame.get_pixel(x, y)))
	}
}
