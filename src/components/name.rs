use id_alloc::{IdTrait};

use world::{World};
use entity::{Entity};

#[derive(Debug)]
pub struct Name {
    name: &'static str,
}

impl Name {
    pub fn new<I: IdTrait, T: Entity<I, T>>(name: &'static str, id: I, world: &mut World<I, T>) -> Name {
        world.register_name(id, name);
        Name {
            name: name,
        }
    }
}
