
use glium;
use glium::glutin;
use image;

use std::str;
use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use super::{Frame, Texture};
use super::error::*;

/// Represents the dimension mode
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DimensionMode {
	/// The draw units are in pixels with the top left being the origin.
	Pixel,

	/// The draw units are 0.0 - 1.0 with the center being the origin.
	Raw,

	/// Same as `Raw` except the top left is the origin and the X axis is scaled
	/// to match the window's aspect ratio. (ie. y = (0.0..1.0) and x =
	/// (0.0..aspect))
	Scaled,
}

/// Represents a `glium::Display` but also stores shaders and manages frames.
pub struct Window {
	pub(crate) display: glium::Display,
	pub(crate) col_shader: glium::Program,
	pub(crate) tex_shader: glium::Program,
	pub(crate) dim_mode: DimensionMode,
}

impl Window {
	/// Create a new `Window` by creating a new `glium::Display`, binding an
	/// OpenGL context to it, and loading the color and texture shaders.
	pub fn new<S: Into<String>>(
		title: S,
		width: u32,
		height: u32,
		events_loop: &glutin::EventsLoop,
	) -> Result<Self> {
		let window = glutin::WindowBuilder::new()
			.with_title(title)
			.with_dimensions(width, height);
		let context = glutin::ContextBuilder::new();
		let display = glium::Display::new(window, context, &events_loop)?;

		let col_shader = program!(&display, 140 => {
			vertex: include_str!("shaders/color.vert"),
			fragment: include_str!("shaders/color.frag"),
		})?;

		let tex_shader = program!(&display, 140 => {
			vertex: include_str!("shaders/tex.vert"),
			fragment: include_str!("shaders/tex.frag"),
		})?;

		Ok(Window {
			display: display,
			col_shader: col_shader,
			tex_shader: tex_shader,
			dim_mode: DimensionMode::Raw,
		})
	}

	/// Set the `DimensionMode` for all future `Frame`'s.
	pub fn with_dim_mode(self, dim_mode: DimensionMode) -> Self {
		Window {
			dim_mode: dim_mode,
			..self
		}
	}

	/// Generate a new `Frame` that represents the next draw cycle. The frame
	/// will be pushed to the screen when `Frame::finish()` is called.
	pub fn next_frame(&self) -> Frame {
		Frame::new(&self)
	}

	/// Get a reference to the underlying `glium::Display`.
	pub fn display(&self) -> &glium::Display {
		&self.display
	}

	/// Load an image from a file and bind it to a glium `Texture2d`.
	pub fn load_image<P: AsRef<Path>>(&self, path: P) -> Result<Rc<Texture>> {
		let mut buf = Vec::new();
		File::open(path).unwrap().read_to_end(&mut buf).unwrap();
		let img = image::load_from_memory(&buf[..]).unwrap().to_rgba();
		let dims = img.dimensions();
		let texture = Texture::new(
			&self.display,
			glium::texture::RawImage2d::from_raw_rgba(img.into_raw(), dims),
		)?;
		Ok(Rc::new(texture))
	}
}
