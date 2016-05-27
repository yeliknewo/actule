use piston_window::{polygon, Context, G2d};
use piston_window::math::{Matrix2d, Vec2d, identity, multiply};
use piston_window::types::{Color};

use nalgebra::{Vector2};

use utils::*;

#[derive(Debug)]
pub struct Renderable {
    layer: Layer,
    color: Color,
    polygon: Vec<Vec2d>,
    matrix: Matrix2d,
}

impl Renderable {
    pub fn new(layer: Layer, polygon: Vec<Vector2<Coord>>, color: Color) -> Renderable {
        let mut polygon_piston = vec!();
        for point in polygon {
            polygon_piston.push([point.x, point.y]);
        }
        Renderable {
            layer: layer,
            color: color,
            polygon: polygon_piston,
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

    pub fn get_polygon(&self) -> Vec<Vector2<Coord>> {
        let mut polygon = vec!();
        for point in self.polygon.iter() {
            polygon.push(Vector2::new(point[0], point[1]));
        }
        polygon
    }

    pub fn get_polygon_piston(&self) -> &Vec<Vec2d> {
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

    pub fn set_polygon(&mut self, polygon: Vec<Vector2<Coord>>) {
        let mut polygon_piston = vec!();
        for point in polygon {
            polygon_piston.push([point.x, point.y]);
        }
        self.polygon = polygon_piston;
    }

    pub fn set_polygon_piston(&mut self, polygon: Vec<Vec2d>) {
        self.polygon = polygon;
    }

    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = layer;
    }
}
