use piston_window::{Context, G2d};

use utils::{Layer};

pub struct Renderable {
    layer: Layer,
}

impl Renderable {
    pub fn new(layer: Layer) -> Renderable {
        Renderable {
            layer: layer,
        }
    }

    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        
    }

    pub fn get_layer(&self) -> Layer {
        self.layer
    }
}
