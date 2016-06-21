use piston_window::{polygon, Context, G2d};
use piston_window::math::{Matrix2d, Vec2d, identity, multiply};
use piston_window::types::{Color};

use nalgebra::{Vector2};

use actule::*;

#[derive(Debug)]
pub struct Shape {
    color: Color,
    polygon: Vec<Vec2d>,
    matrix: Matrix2d,
}

impl Shape {
    #[inline]
    pub fn new(polygon: Vec<Vector2<Coord>>, color: Color) -> Shape {
        let mut polygon_piston = vec!();
        for point in polygon {
            polygon_piston.push([point.x, point.y]);
        }
        Shape {
            color: color,
            polygon: polygon_piston,
            matrix: identity(),
        }
    }

    #[inline]
    pub fn draw_2d(&self, c: Context, g: &mut G2d) {
        polygon(self.color, &self.polygon, multiply(c.transform, self.matrix), g);
    }

    #[inline]
    pub fn get_matrix(&self) -> &Matrix2d {
        &self.matrix
    }

    #[inline]
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    #[inline]
    pub fn get_polygon(&self) -> Vec<Vector2<Coord>> {
        let mut polygon = vec!();
        for point in self.polygon.iter() {
            polygon.push(Vector2::new(point[0], point[1]));
        }
        polygon
    }

    #[inline]
    pub fn get_polygon_piston(&self) -> &Vec<Vec2d> {
        &self.polygon
    }

    #[inline]
    pub fn set_matrix(&mut self, matrix: Matrix2d) {
        self.matrix = matrix;
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    #[inline]
    pub fn set_polygon(&mut self, polygon: Vec<Vector2<Coord>>) {
        let mut polygon_piston = vec!();
        for point in polygon {
            polygon_piston.push([point.x, point.y]);
        }
        self.polygon = polygon_piston;
    }

    #[inline]
    pub fn set_polygon_piston(&mut self, polygon: Vec<Vec2d>) {
        self.polygon = polygon;
    }
}
