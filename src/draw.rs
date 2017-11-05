
use cgm;
use std::rc::Rc;
use super::{Texture, Vertex};

/// Trait for drawing colored shapes and b
pub trait Draw {
	/// Draw a single colored triangle strip from `points`.
	fn draw_shape<I: IntoIterator<Item = Vertex>>(&mut self, points: I);

	/// Draw a single textured triangle strip from `points`.
	fn draw_image<I: IntoIterator<Item = Vertex>, T: Into<Rc<Texture>>>(
		&mut self,
		points: I,
		texture: T,
	);
}

/// Trait for transformations
pub trait Transform {
	fn translate(self, transform: cgm::Vector2<f64>) -> Self;

	/// Rotation is in radians
	fn rotate(self, f64) -> Self;

	fn scale(self, transform: cgm::Vector2<f64>) -> Self;
}
