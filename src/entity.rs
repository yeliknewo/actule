use id_alloc::{IdTrait, Node};

use world::{World};
use components::{Renderable, Transform};

pub trait Entity<I: IdTrait, T: Entity<I, T>> {
    fn get_id(&self) -> I;
    fn get_renderable(&self) -> Option<&Box<Renderable>>;
    fn get_transform(&self) -> Option<&Box<Transform>>;
    fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>>;
    fn get_mut_transform(&mut self) -> Option<&mut Box<Transform>>;
    fn tick(&mut self, dt: f64, manager: &mut Node<I>, world: &mut World<I, T>);
    fn is_tick(&self) -> bool;
}
