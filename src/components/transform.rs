use piston_window::math::{Vec2d, multiply, translate, rotate_radians, scale};

use components::*;

#[derive(Debug)]
pub struct Transform {
    position: Vec2d,
    rotation: f64,
    scale: Vec2d,
    dirty: bool,
}

impl Transform {
    pub fn new(position: Vec2d, rotation: f64, scale: Vec2d) -> Transform {
        Transform {
            position: position,
            rotation: rotation,
            scale: scale,
            dirty: true,
        }
    }

    pub fn tick(&mut self, renderable: &mut Box<Renderable>) {
        if self.dirty {
            renderable.set_matrix(multiply(multiply(translate(self.position), rotate_radians(self.rotation)), scale(self.scale[0], self.scale[1])));
            self.dirty = false;
        }
    }

    pub fn set_position(&mut self, position: Vec2d) {
        self.position = position;
        self.dirty = true;
    }

    pub fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation;
        self.dirty = true;
    }

    pub fn set_scale(&mut self, scale: Vec2d) {
        self.scale = scale;
        self.dirty = true;
    }

    pub fn get_position(&self) -> &Vec2d {
        &self.position
    }

    pub fn get_rotation(&self) -> f64 {
        self.rotation
    }

    pub fn get_scale(&self) -> &Vec2d {
        &self.scale
    }
}
