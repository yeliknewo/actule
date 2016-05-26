use std::hash::Hash;

use piston_window::{PistonWindow, clear, UpdateEvent, BuildFromWindowSettings, Window, AdvancedWindow, OpenGLWindow, GenericEvent};
use id_alloc::*;

use entity::{Entity};
use world::{World};

#[derive(Debug)]
pub struct Game<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> {
    world: World<I, T>,
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> Game<I, T> {
    pub fn new() -> Game<I, T> {
        Game {
            world: World::new(),
        }
    }

    #[inline]
    pub fn get_world(&self) -> &World<I, T> {
        &self.world
    }

    #[inline]
    pub fn get_mut_world(&mut self) -> &mut World<I, T> {
        &mut self.world
    }

    pub fn run<W>(&mut self, manager: &mut Node<I>, window: &mut PistonWindow<W>) where W: BuildFromWindowSettings + Window + AdvancedWindow + OpenGLWindow, W::Event: GenericEvent {
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                for layer in self.world.get_active_layers() {
                    for entity_id in self.world.get_render_ids().get(layer).expect("Active layer wasn't a layer").iter() {
                        if let Some(entity) = self.world.get_entity_by_id(*entity_id) {
                            if let Some(renderable) = entity.get_renderable() {
                                renderable.draw_2d(c, g);
                            }
                        }
                    }
                }
            });
            if let Some(args) = e.update_args() {
                if let Some(tick_ids) = self.world.take_tick_ids() {
                    for id in tick_ids.iter() {
                        if let Some(mut entity) = self.world.take_entity_by_id(*id) {
                            entity.tick(args.dt, manager, &mut self.world);
                            self.world.give_entity(entity);
                        }
                    }
                    self.world.give_tick_ids(tick_ids);
                }

            }
        }
    }
}
