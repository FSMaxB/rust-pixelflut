use crate::frame_serializer::FrameSerializer;
use image::imageops::FilterType;
use image::{DynamicImage, RgbaImage};

// TODO: Find better API with better separation of concerns
pub trait FramePainter {
	fn update_frame(&mut self, frame: RgbaImage);
	fn update_dimensions(&mut self, x: u32, y: u32);
	fn update_resize_type(&mut self, resize_type: ResizeType);
	fn update_resize_filter(&mut self, resize_filter: FilterType);
	fn update_position(&mut self, x: u32, y: u32);
	fn update_stream_count(&mut self, count: usize);
	fn update_serializer(&mut self, serializer: Box<dyn FrameSerializer>);
}

pub struct FrameResizer {
	original_frame: RgbaImage,
	resized_frame: RgbaImage,
	dimensions: (u32, u32),
	resize_type: ResizeType,
	resize_filter: FilterType,
}

impl From<RgbaImage> for FrameResizer {
	fn from(frame: RgbaImage) -> Self {
		let dimensions = frame.dimensions();
		Self {
			original_frame: frame.clone(),
			resized_frame: frame,
			dimensions,
			resize_type: ResizeType::Stretch,
			resize_filter: FilterType::Lanczos3,
		}
	}
}

impl FrameResizer {
	pub fn frame(&self) -> &RgbaImage {
		&self.resized_frame
	}

	pub fn update_frame(&mut self, frame: RgbaImage) {
		self.original_frame = frame;
		self.resize();
	}

	pub fn update_dimensions(&mut self, x: u32, y: u32) {
		self.dimensions = (x, y);
		self.resize();
	}

	pub fn update_type(&mut self, resize_type: ResizeType) {
		self.resize_type = resize_type;
		self.resize();
	}

	pub fn update_filter(&mut self, resize_filter: FilterType) {
		self.resize_filter = resize_filter;
		self.resize();
	}

	fn resize(&mut self) {
		let (x, y) = self.dimensions;
		self.resized_frame = resize_frame(self.original_frame.clone(), self.resize_type, self.resize_filter, x, y);
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
	frame: RgbaImage,
	resize_type: ResizeType,
	resize_filter: FilterType,
	width: u32,
	height: u32,
) -> RgbaImage {
	let dynamic_image = DynamicImage::from(frame);
	let resized_image = match resize_type {
		ResizeType::Crop => dynamic_image.resize_to_fill(width, height, resize_filter),
		ResizeType::Stretch => dynamic_image.resize_exact(width, height, resize_filter),
		ResizeType::Fill => dynamic_image.resize(width, height, resize_filter),
	};

	resized_image.into_rgba8()
}
