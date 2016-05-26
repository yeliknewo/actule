use piston_window::{polygon, Context, G2d};
use piston_window::math::{Matrix2d, Vec2d, identity, multiply};
use piston_window::types::{Color};

use utils::{Layer};

#[derive(Debug)]
pub struct Renderable {
    layer: Layer,
    color: Color,
    polygon: Vec<Vec2d>,
    matrix: Matrix2d,
}

impl Renderable {
    pub fn new(layer: Layer, polygon: Vec<Vec2d>, color: Color) -> Renderable {
        Renderable {
            layer: layer,
            color: color,
            polygon: polygon,
            matrix: identity(),
        }
    }

    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        polygon(self.color, &self.polygon, multiply(c.transform, self.matrix), g);
    }

    pub fn get_matrix(&self) -> &Matrix2d {
        &self.matrix
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_polygon(&self) -> &Vec<Vec2d> {
        &self.polygon
    }

    pub fn get_layer(&self) -> Layer {
        self.layer
    }

    pub fn set_matrix(&mut self, matrix: Matrix2d) {
        self.matrix = matrix;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_polygon(&mut self, polygon: Vec<Vec2d>) {
        self.polygon = polygon;
    }

    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = layer;
    }
}
