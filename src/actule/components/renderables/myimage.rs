use piston_window::{Context, G2d, Texture, TextureSettings};
use gfx_device_gl::{Resources, Factory};
use piston_window::image;

pub struct MyImage {
    texture: Texture<Resources>
}

impl MyImage {
    //creates a new texture from memory
    //use include_bytes!
    //images can be created from paths, etc etc.  that can be added as needed?
    pub fn new(factory: &mut Factory, buffer: &[u8], width: u32, height: u32) -> MyImage {
        let texture = Texture::from_memory_alpha(factory, buffer, width, height, &TextureSettings::new()).unwrap();
        MyImage {
            texture: texture
        }
    }
    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        image(&self.texture, c.transform, g);
    }
}
