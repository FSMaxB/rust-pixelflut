use crate::frame_serializer::FrameSerializer;
use crate::{Coordinate, Dimension};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, RgbaImage};
use std::convert::TryInto;

pub mod io_uring;

// TODO: Find better API with better separation of concerns
pub trait FramePainter {
	fn update_frame(&mut self, frame: DynamicImage);
	fn update_dimensions(&mut self, dimensions: Dimension);
	fn update_resize_type(&mut self, resize_type: ResizeType);
	fn update_resize_filter(&mut self, resize_filter: FilterType);
	fn update_position(&mut self, coordinate: Coordinate);
	fn update_stream_count(&mut self, count: usize);
	fn update_serializer(&mut self, serializer: Box<dyn FrameSerializer + 'static>);
}

pub struct FrameResizer {
	original_frame: DynamicImage,
	resized_frame: RgbaImage,
	dimensions: (u32, u32),
	resize_type: ResizeType,
	resize_filter: FilterType,
}

impl From<DynamicImage> for FrameResizer {
	fn from(frame: DynamicImage) -> Self {
		let dimensions = frame.dimensions();
		let resized_frame = frame.to_rgba8();
		Self {
			original_frame: frame,
			resized_frame,
			dimensions,
			resize_type: ResizeType::Stretch,
			resize_filter: FilterType::Lanczos3,
		}
	}
}

impl FrameResizer {
	pub fn resized_frame(&self) -> RgbaImage {
		self.resized_frame.clone()
	}

	pub fn update_frame(&mut self, frame: DynamicImage) -> RgbaImage {
		self.original_frame = frame;
		self.resize()
	}

	pub fn update_dimensions(&mut self, dimensions: Dimension) -> RgbaImage {
		self.dimensions = (
			dimensions.width.try_into().unwrap(),
			dimensions.height.try_into().unwrap(),
		);
		self.resize()
	}

	pub fn update_type(&mut self, resize_type: ResizeType) -> RgbaImage {
		self.resize_type = resize_type;
		self.resize()
	}

	pub fn update_filter(&mut self, resize_filter: FilterType) -> RgbaImage {
		self.resize_filter = resize_filter;
		self.resize()
	}

	fn resize(&mut self) -> RgbaImage {
		let (x, y) = self.dimensions;
		self.resized_frame = resize_frame(self.original_frame.clone(), self.resize_type, self.resize_filter, x, y);
		self.resized_frame.clone()
	}
}

#[derive(Debug, Clone, Copy)]
pub enum ResizeType {
	Crop,
	Stretch,
	Fill,
}

impl Default for ResizeType {
	fn default() -> Self {
		Self::Stretch
	}
}

fn resize_frame(
	frame: DynamicImage,
	resize_type: ResizeType,
	resize_filter: FilterType,
	width: u32,
	height: u32,
) -> RgbaImage {
	let resized_image = match resize_type {
		ResizeType::Crop => frame.resize_to_fill(width, height, resize_filter),
		ResizeType::Stretch => frame.resize_exact(width, height, resize_filter),
		ResizeType::Fill => frame.resize(width, height, resize_filter),
	};

	resized_image.into_rgba8()
}
