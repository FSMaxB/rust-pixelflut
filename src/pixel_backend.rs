use std::future::Future;

pub trait PixelBackend {
	type Future: Future<Output = anyhow::Result<()>>;
}
