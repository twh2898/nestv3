
extern crate cgmath as cgm;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate glium;
extern crate image;

mod window;
mod frame;
mod vertex;
mod draw;

pub use window::*;
pub use frame::*;
pub use vertex::*;
pub use draw::*;

use std::time::Duration;

/// Texture is the glum::texture that is currently being used, `SrgbTexture2d` or `Texture2d`.
pub type Texture = glium::texture::SrgbTexture2d;

/// Combines errors from multiple libraries into a single type for ease of use.
mod error {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Image(super::image::ImageError);
            Texture(super::glium::texture::TextureCreationError);
            Program(super::glium::program::ProgramChooserCreationError);
            DisplayCreation(super::glium::backend::glutin::DisplayCreationError);
        }
    }
}
pub use error::*;

/// Return a `Duration` represented as `f64` decimal seconds.
pub fn as_sec(elapsed: Duration) -> f64 {
    elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1000000000.0
}
