
use cgm;
use glium;
use glium::Surface;
use std::rc::Rc;
use super::{Draw, Texture, Vertex, Window, DimensionMode};

/// Represents a single draw cycle for a `Window`.
pub struct Frame<'a> {
	window: &'a Window,
	target: glium::Frame,
}

impl<'a> Frame<'a> {
	/// Create a new `Frame` for the current `Window`. This frame represents a
	/// single draw cycle that will be completed when `Frame::finish()` is
	/// called.
	///
	/// *Note* you must call `finish()` on any existing frame before creating a
	/// new one
	pub fn new(window: &'a Window) -> Self {
		Frame {
			window: window,
			target: window.display.draw(),
		}
	}

	/// Clears the screen to black.
	pub fn clear(&mut self) {
		self.clear_color([0.0, 0.0, 0.0, 1.0]);
	}

	/// Clears the screen to a specific color
	pub fn clear_color<C: Into<[f32; 4]>>(&mut self, color: C) {
		let color = color.into();
		self.target
			.clear_color(color[0], color[1], color[2], color[3]);
	}

	/// Completes the draw cycle for this frame
	pub fn finish(self) -> Result<(), glium::SwapBuffersError> {
		self.target.finish()
	}
}

impl<'a> Draw for Frame<'a> {
	fn draw_shape<I: IntoIterator<Item = Vertex>>(&mut self, points: I) {
		let (width, height) = self.target.get_dimensions();
		let width = width as f32;
		let height = height as f32;

		let display = &self.window.display;
		let shader = &self.window.col_shader;
		let v = glium::VertexBuffer::new(display, &points.into_iter().map(|p| p.into()).collect::<Vec<Vertex>>())
			.unwrap();
		let i = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

		let (left, right, bottom, top) = match self.window.dim_mode {
			DimensionMode::Pixel => (0.0, width, height, 0.0),
			DimensionMode::Raw => (-1.0, 1.0, -1.0, 1.0),
			DimensionMode::Scaled => (0.0, width / height, 1.0, 0.0),
		};
		let u = uniform!{
			perspective: Into::<[[f32; 4]; 4]>::into(cgm::ortho(left, right, bottom, top, 0.0, 1.0)),
		};

		let _ = self.target.draw(&v, &i, shader, &u, &Default::default());
	}

	fn draw_image<I: IntoIterator<Item = Vertex>, T: Into<Rc<Texture>>>(
		&mut self,
		points: I,
		texture: T,
	) {
		let texture = texture.into();

		let (width, height) = self.target.get_dimensions();
		let width = width as f32;
		let height = height as f32;

		let display = &self.window.display;
		let shader = &self.window.tex_shader;
		let v = glium::VertexBuffer::new(display, &points.into_iter().map(|p| p.into()).collect::<Vec<Vertex>>())
			.unwrap();
		let i = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
		
		let (left, right, bottom, top) = match self.window.dim_mode {
			DimensionMode::Pixel => (0.0, width, height, 0.0),
			DimensionMode::Raw => (-1.0, 1.0, -1.0, 1.0),
			DimensionMode::Scaled => (0.0, width / height, 1.0, 0.0),
		};
		let u = uniform!{
			perspective: Into::<[[f32; 4]; 4]>::into(cgm::ortho(left, right, bottom, top, 0.0, 1.0)),
			texture: glium::uniforms::Sampler::new(texture.as_ref())
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest).minify_filter(glium::uniforms::MinifySamplerFilter::NearestMipmapLinear).wrap_function(glium::uniforms::SamplerWrapFunction::Repeat),
		};

		let _ = self.target.draw(&v, &i, shader, &u, &Default::default());
	}
}
