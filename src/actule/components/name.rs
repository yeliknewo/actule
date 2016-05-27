use std::hash::Hash;

use id_alloc::*;

use actule::*;

#[derive(Debug)]
pub struct Name {
    name: &'static str,
}

impl Name {
    pub fn new<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>>(name: &'static str, id: I, world: &mut World<I, T>) -> Name {
        world.register_name(id, name);
        Name {
            name: name,
        }
    }
}
