use piston_window::math::{Vec2d, multiply, translate, rotate_radians, scale};
use nalgebra::{Vector1, Vector2, Isometry2, Rotation, Translation};

use components::*;
use utils::*;

#[derive(Debug)]
pub struct Transform {
    isometry2: Isometry2<Coord>,
    scale: Vector2<Coord>,
    dirty: bool,
}

impl Transform {
    pub fn new(position: Vec2d, rotation: Coord, scale: Vec2d) -> Transform {
        Transform {
            isometry2: Isometry2::new(Vector2::new(position[0], position[1]), Vector1::new(rotation)),
            scale: Vector2::new(scale[0], scale[1]),
            dirty: true,
        }
    }

    pub fn tick(&mut self, renderable: &mut Box<Renderable>) {
        if self.dirty {
            renderable.set_matrix(
                multiply(
                    multiply(
                        translate([self.isometry2.translation.x, self.isometry2.translation.y]),
                        rotate_radians(self.isometry2.rotation.rotation().x)
                    ),
                    scale(self.scale.x, self.scale.y)
                )
            );
            self.dirty = false;
        }
    }

    pub fn get_isometry(&self) -> &Isometry2<Coord> {
        &self.isometry2
    }

    pub fn get_scale(&self) -> &Vector2<Coord> {
        &self.scale
    }

    pub fn get_mut_isometry(&mut self) -> &mut Isometry2<Coord> {
        &mut self.isometry2
    }

    pub fn get_mut_scale(&mut self) -> &mut Vector2<Coord> {
        &mut self.scale
    }
}
