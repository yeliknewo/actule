use super::renderables::*;
use actule::utils::Layer;
use piston_window::{Context, G2d};
use piston_window::math::{Matrix2d, identity};

pub struct Renderable {
    //since a shape and an image both have polygons, maybe that should be kept here
    layer: Layer,
    matrix: Matrix2d,
    shape: Option<Shape>,
    image: Option<MyImage>
}

impl Renderable {
    pub fn new(layer: Layer) -> Renderable {
        Renderable {
            layer: layer,
            matrix: identity(),
            shape: None,
            image: None,
        }
    }
    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        if self.shape.is_some() {
            self.shape.as_ref().unwrap().draw_2d(c, g, self.matrix);
        }
        if self.image.is_some() {
            self.image.as_ref().unwrap().draw_2d(c, g, self.matrix);
        }
    }

    pub fn get_layer(&self) -> Layer {
        self.layer
    }
    pub fn get_shape(&self) -> Option<&Shape> {
        self.shape.as_ref()
    }
    pub fn get_mut_shape(&mut self) -> Option<&mut Shape> {
        self.shape.as_mut()
    }
    pub fn get_image(&self) -> Option<&MyImage> {
        self.image.as_ref()
    }
    pub fn get_mut_image(&mut self) -> Option<&mut MyImage> {
        self.image.as_mut()
    }
    pub fn get_matrix(&self) -> &Matrix2d {
        &self.matrix
    }

    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = layer;
    }
    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = Some(shape);
    }
    pub fn set_image(&mut self, image: MyImage) {
        self.image = Some(image);
    }
    pub fn set_matrix(&mut self, matrix: Matrix2d) {
        self.matrix = matrix;
    }

    pub fn with_shape(mut self, shape: Shape) -> Renderable {
        self.shape = Some(shape);
        self
    }
    pub fn with_image(mut self, image: MyImage) -> Renderable {
        self.image = Some(image);
        self
    }
}
