
use glium;
use std::rc::Rc;

/// Texture is the glum::texture that is currently being used, `SrgbTexture2d` or `Texture2d`.
pub type Texture = glium::texture::SrgbTexture2d;

/// Represents a TextureAtlas mapping
#[derive(Clone, Debug)]
pub struct TextureAtlas {
	texture: Rc<Texture>,
	cols: u8,
	rows: u8,
}

impl TextureAtlas {
	/// Creates a new `TextureAtlas` with an existing `Texture` and the
	/// specified number of columns and rows. The texture is never used but is
	/// stored for convenience of association.
	pub fn new<T: Into<Rc<Texture>>>(texture: T, columns: u8, rows: u8) -> Self {
		TextureAtlas {
			texture: texture.into(),
			cols: columns,
			rows: rows,
		}
	}

	/// Get the uvw coordinates for a texture at `id`. This location is
	/// calculated as `row = id / cols` and `col = id % cols`. If the row or col
	/// is out of bounds of the texture, the default uvw of `[0.0, 0.0, 1.0,
	/// 1.0]` will be returned.
	///
	/// _Note_ `id` indexes at 1 rather than 0. An `id` of 0 will return the
	/// default uvw of `[0.0, 0.0, 1.0, 1.0]`.
	pub fn uvw(&self, id: u16) -> [f32; 4] {
		if id == 0 || id >= (self.rows as u16) * (self.cols as u16) {
			return [0.0, 0.0, 1.0, 1.0];
		}

		let id = id - 1;
		let row = id / self.rows as u16;
		let col = id % self.cols as u16;

		let u1 = col as f32 / self.cols as f32;
		let u2 = (col + 1) as f32 / self.cols as f32;
		let v1 = row as f32 / self.rows as f32;
		let v2 = (row + 1) as f32 / self.rows as f32;

		[u1, v1, u2, v2]
	}


	/// Get an `Rc` of the encapsulated `Texture`.
	pub fn get_tex(&self) -> Rc<Texture> {
		self.texture.clone()
	}
}
