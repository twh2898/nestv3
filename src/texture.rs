
use glium;

/// Texture is the glum::texture that is currently being used, `SrgbTexture2d` or `Texture2d`.
pub type Texture = glium::texture::SrgbTexture2d;

/// Represents a TextureAtlas mapping
#[derive(Clone, Debug)]
pub struct TextureAtlas {
	cols: u8,
	rows: u8,
}

impl TextureAtlas {
	/// Creates a new `TextureAtlas` with a specified number of columns and
	/// rows.
	pub fn new(columns: u8, rows: u8) -> Self {
		TextureAtlas {
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
		if id == 0 || id > (self.rows as u16) * (self.cols as u16) {
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
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_uvw() {
		let atlas = TextureAtlas::new(4, 4);
		assert_eq!([0.0, 0.0, 1.0, 1.0], atlas.uvw(0));
		assert_eq!([0.0, 0.0, 1.0, 1.0], atlas.uvw(17));
		assert_eq!([0.0, 0.0, 0.25, 0.25], atlas.uvw(1));
	}
}
