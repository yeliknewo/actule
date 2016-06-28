use piston_window::{Context, G2d};
pub use gfx_device_gl::{Resources, Factory};
use piston_window::image::*;

use piston_window::{Texture, Flip, TextureSettings};
use std::path::Path;

use piston_window::math::{Matrix2d, identity, multiply};
use piston_window::image;

use piston_window::ImageSize;
use piston_window::Transformed;
//use piston_window::DrawState;


pub struct MyImage {
    matrix: Matrix2d,
    texture: Texture<Resources>,
    //dimensions: [f64; 4], //[x, y, w, h]
    //image: Image
}

impl MyImage {
    pub fn new(factory: &mut Factory, path: &str, dimensions: [f64; 4]) -> MyImage {
        //let image = Image::new().rect(dimensions);
        let texture = Texture::from_path(factory, Path::new(path), Flip::None, &TextureSettings::new()).unwrap();

        MyImage {
            matrix: identity(),
            texture: texture,
            //dimensions: dimensions,
            //image: image
        }
    }

    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        let (spritex, spritey) = self.texture.get_size();
        image(&self.texture, multiply(c.transform.trans(spritex as f64, spritey as f64), self.matrix), g);
    }
}
