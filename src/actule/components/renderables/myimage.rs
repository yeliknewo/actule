use piston_window::{Context, G2d, Texture, TextureSettings};
pub use gfx_device_gl::{Resources, Factory};
use piston_window::image::*;
//use piston_window::DrawState;


pub struct MyImage {
    texture: Texture<Resources>,
    dimensions: [u32; 4], //[x, y, w, h]
    image: Image
}

impl MyImage {
    //creates a new texture from memory
    //use include_bytes!
    //images can be created from paths, etc etc.  that can be added as needed?
    pub fn new(factory: &mut Factory, buffer: &[u8], width: u32, height: u32, dimensions: [u32; 4]) -> MyImage {
        let texture = Texture::from_memory_alpha(factory, buffer, width, height, &TextureSettings::new()).unwrap();
        let image = Image::new().rect([dimensions[0] as f64, dimensions[1] as f64, dimensions[2] as f64, dimensions[3] as f64]);
        MyImage {
            texture: texture,
            dimensions: dimensions,
            image: image
        }
    }
    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        self.image.draw(&self.texture, &c.draw_state, c.transform, g);
    }
}
