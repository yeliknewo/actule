use piston_window::{Context, G2d, Texture, TextureSettings};
pub use gfx_device_gl::{Resources, Factory};
use piston_window::image;
//use piston_window::DrawState;


pub struct MyImage {
    texture: Texture<Resources>,
    dimensions: [u32; 4] //[x, y, w, h]
}

impl MyImage {
    //creates a new texture from memory
    //use include_bytes!
    //images can be created from paths, etc etc.  that can be added as needed?
    pub fn new(factory: &mut Factory, buffer: &[u8], width: u32, height: u32, dimensions: [u32; 4]) -> MyImage {
        let texture = Texture::from_memory_alpha(factory, buffer, width, height, &TextureSettings::new()).unwrap();
        MyImage {
            texture: texture,
            dimensions: dimensions
        }
    }
    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        //this is not efficient at all!
        //i think this breaks stuff
        let temp_c = c;
        temp_c.draw_state.scissor(self.dimensions);
        image(&self.texture, temp_c.transform, g);
    }
}
