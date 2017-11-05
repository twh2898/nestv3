
/// Type alias for `[f32; 4]`.
pub type Color = [f32; 4];

/// Represents a single vertex with position, color, and uvw coordinates.
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
	position: [f64; 2],
	color: [f32; 4],
	uvw: [f32; 2],
}
implement_vertex!(Vertex, position, color, uvw);

impl From<([f64; 2], [f32; 4], [f32; 2])> for Vertex {
	fn from(data: ([f64; 2], [f32; 4], [f32; 2])) -> Self {
		Vertex {
			position: data.0,
			color: data.1,
			uvw: data.2,
		}
	}
}

impl From<(f64, f64, f32, f32)> for Vertex {
	fn from(data: (f64, f64, f32, f32)) -> Self {
		Vertex {
			position: [data.0, data.1],
			color: [1.0; 4],
			uvw: [data.2, data.3],
		}
	}
}


impl From<(f64, f64, [f32; 4])> for Vertex {
	fn from(data: (f64, f64, [f32; 4])) -> Self {
		Vertex {
			position: [data.0, data.1],
			color: data.2,
			uvw: [0.0; 2],
		}
	}
}
