use piston_window::{Context, G2d};
pub use gfx_device_gl::{Resources, Factory};
use piston_window::image::*;

use piston_window::{Texture, Flip, TextureSettings};
use std::path::Path;

use piston_window::math::{Matrix2d, multiply};
use piston_window::image;


pub struct MyImage {
    texture: Texture<Resources>,
}

impl MyImage {
    pub fn new(factory: &mut Factory, path: &Path) -> MyImage {
        let texture = Texture::from_path(factory, path, Flip::None, &TextureSettings::new()).unwrap();

        MyImage {
            texture: texture,
        }
    }

    pub fn draw_2d(&self, c: Context, g: &mut G2d, matrix: Matrix2d) {
        image(&self.texture, multiply(c.transform, matrix), g);
    }
}
