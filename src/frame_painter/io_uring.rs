use super::FrameResizer;
use crate::frame_painter::{FramePainter, ResizeType};
use crate::frame_serializer::{FrameSerializer, RandomSerializer};
use crate::{Coordinate, Dimension, Pixel};
use anyhow::{anyhow, bail, Context};
use image::imageops::FilterType;
use image::{DynamicImage, RgbaImage};
use std::convert::Infallible;
use std::io::Write;
use std::net::SocketAddr;
use std::thread;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio_uring::buf::IoBuf;
use tokio_uring::net::TcpStream;

pub struct IoUringFramePainter {
	resizer: FrameResizer,
	update_sender: mpsc::Sender<Update>,
	serializer: Box<dyn FrameSerializer + 'static>,
	stream_count: usize,
	position: Coordinate,
}

impl IoUringFramePainter {
	pub fn start(ip: SocketAddr, frame: DynamicImage) -> IoUringFramePainter {
		let (update_sender, update_receiver) = mpsc::channel(2);
		let _ = thread::spawn(move || tokio_uring::start(run_io(ip, update_receiver)));
		Self {
			resizer: FrameResizer::from(frame),
			update_sender,
			serializer: Box::new(RandomSerializer::default()),
			stream_count: 0,
			position: Coordinate::default(),
		}
	}
}

impl FramePainter for IoUringFramePainter {
	fn update_frame(&mut self, frame: DynamicImage) {
		let frame = self.resizer.update_frame(frame);
		let _ = self.update_sender.try_send(Update {
			frame,
			serializer: self.serializer.duplicate(),
			stream_count: self.stream_count,
			position: self.position,
		});
	}

	fn update_dimensions(&mut self, dimensions: Dimension) {
		let frame = self.resizer.update_dimensions(dimensions);
		let _ = self.update_sender.try_send(Update {
			frame,
			serializer: self.serializer.duplicate(),
			stream_count: self.stream_count,
			position: self.position,
		});
	}

	fn update_resize_type(&mut self, resize_type: ResizeType) {
		let frame = self.resizer.update_type(resize_type);
		let _ = self.update_sender.try_send(Update {
			frame,
			serializer: self.serializer.duplicate(),
			stream_count: self.stream_count,
			position: self.position,
		});
	}

	fn update_resize_filter(&mut self, resize_filter: FilterType) {
		let frame = self.resizer.update_filter(resize_filter);
		let _ = self.update_sender.try_send(Update {
			frame,
			serializer: self.serializer.duplicate(),
			stream_count: self.stream_count,
			position: self.position,
		});
	}

	fn update_position(&mut self, position: Coordinate) {
		let frame = self.resizer.resized_frame().clone();
		self.position = position;
		let _ = self.update_sender.try_send(Update {
			frame,
			serializer: self.serializer.duplicate(),
			stream_count: self.stream_count,
			position: self.position,
		});
	}

	fn update_stream_count(&mut self, count: usize) {
		let frame = self.resizer.resized_frame().clone();
		self.stream_count = count;
		let _ = self.update_sender.try_send(Update {
			frame,
			serializer: self.serializer.duplicate(),
			stream_count: self.stream_count,
			position: self.position,
		});
	}

	fn update_serializer(&mut self, serializer: Box<dyn FrameSerializer + 'static>) {
		let frame = self.resizer.resized_frame().clone();
		self.serializer = serializer;
		let _ = self.update_sender.try_send(Update {
			frame,
			serializer: self.serializer.duplicate(),
			stream_count: self.stream_count,
			position: self.position,
		});
	}
}

struct Update {
	frame: RgbaImage,
	serializer: Box<dyn FrameSerializer + 'static>,
	stream_count: usize,
	position: Coordinate,
}

async fn run_io(ip: SocketAddr, mut update_receiver: mpsc::Receiver<Update>) -> anyhow::Result<Infallible> {
	let mut senders = Vec::<mpsc::Sender<Vec<u8>>>::new();
	loop {
		let Update {
			frame,
			mut serializer,
			stream_count,
			position,
		} = update_receiver
			.recv()
			.await
			.ok_or_else(|| anyhow!("Update channel closed"))?;
		match senders.len() {
			length if length > stream_count => {
				drop(senders.split_off(stream_count));
			}
			length if length < stream_count => {
				// start missing streams
				for _ in senders.len()..stream_count {
					let (sender, receiver) = mpsc::channel(1);
					senders.push(sender);
					let stream = match TcpStream::connect(ip).await {
						Ok(stream) => stream,
						Err(error) => {
							println!("Connection failed, retrying on next update: {error}");
							continue;
						}
					};
					tokio_uring::spawn(async move { run_single_stream(stream, receiver).await });
				}
			}
			_ => {}
		}

		let mut pixels = serializer.serialize(&frame);
		let pixel_count = frame.pixels().len();
		let pixels_per_stream = pixel_count / stream_count;
		let mut buffers = vec![Vec::<u8>::with_capacity(Pixel::BYTE_ESTIMATE); stream_count];
		for buffer in &mut buffers {
			for _ in 0..pixels_per_stream {
				let mut pixel = pixels.next().context("Pixel count mismatch")?;
				pixel.coordinate += position;
				buffer.write_fmt(format_args!("{pixel}"))?;
			}
		}

		// write remaining pixels
		if let Some(last_buffer) = buffers.last_mut() {
			for mut pixel in pixels {
				pixel.coordinate += position;
				last_buffer.write_fmt(format_args!("{pixel}"))?;
			}
		}
		for (sender, buffer) in senders.iter().zip(buffers) {
			if sender.send(buffer).await.is_err() {
				println!("Broken stream.");
			}
		}
	}
}

async fn run_single_stream(stream: TcpStream, mut receiver: mpsc::Receiver<Vec<u8>>) -> anyhow::Result<Infallible> {
	let mut buffer = receiver
		.recv()
		.await
		.ok_or_else(|| anyhow!("channel closed"))?
		.slice(..);
	loop {
		if buffer.is_empty() {
			match receiver.try_recv() {
				// New buffer is available
				Ok(bytes) => buffer = bytes.slice(..),
				// Start the same buffer over from the beginning
				Err(TryRecvError::Empty) => {
					// Everything was written, start from the beginning
					buffer = buffer.into_inner().slice(..);
				}
				Err(TryRecvError::Disconnected) => bail!("Stream stopped."),
			}
		}

		buffer = {
			let (result, buffer) = stream.write(buffer).await;
			result?;
			buffer
		};
	}
}
