use piston_window::{Context, G2d};
pub use gfx_device_gl::{Resources, Factory};
use piston_window::image::*;

use piston_window::{Texture, Flip, TextureSettings};
use std::path::Path;

use piston_window::math::{Matrix2d, multiply};
use piston_window::image;
//use piston_window::DrawState;


pub struct MyImage {
    texture: Texture<Resources>,
    //dimensions: [f64; 4], //[x, y, w, h]
    image: Image
}

impl MyImage {
    pub fn new(factory: &mut Factory, path: &Path, dimensions: [f64; 4]) -> MyImage {
        let image = Image::new().rect(dimensions);
        let texture = Texture::from_path(factory, path, Flip::None, &TextureSettings::new()).unwrap();

        MyImage {
            texture: texture,
            //dimensions: dimensions,
            image: image
        }
    }

    pub fn draw_2d(&self, c: Context, g: &mut G2d, matrix: Matrix2d) {
        self.image.draw(&self.texture, &c.draw_state, multiply(c.transform, matrix), g);
    }
}
