use std::hash::Hash;

use piston_window::*;
use id_alloc::*;

use actule::*;

#[derive(Debug)]
pub struct Game<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> {
    world: World<I, T>,
    minput: Minput
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> Game<I, T> {
    #[inline]
    pub fn new(background_color: [f32; 4]) -> Game<I, T> {
        Game {
            world: World::new(background_color),
            minput: Minput::new()
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
                clear(self.world.get_background_color(), g);
                for layer in self.world.get_render_layered().get_active_layers() {
                    for entity_id in self.world.get_render_layered().get_layer(layer).expect("Render layer was none").iter() {
                        if let Some(entity) = self.world.get_entity_by_id(*entity_id) {
                            if let Some(renderable) = entity.get_renderable() {
                                renderable.draw_2d(c, g);
                            }
                        }
                    }
                }
            });
            if let Some(button) = e.press_args() {
                match button {
                    Button::Keyboard(key) => self.minput.set_key(key, KeyState::Pressed),
                    Button::Mouse(mouse_button) => self.minput.set_mouse_button(mouse_button, KeyState::Pressed),
                    Button::Controller(controller_button) => self.minput.set_controller_button(controller_button, KeyState::Pressed),
                }
            }
            if let Some(button) = e.release_args() {
                match button {
                    Button::Keyboard(key) => self.minput.set_key(key, KeyState::Released),
                    Button::Mouse(mouse_button) => self.minput.set_mouse_button(mouse_button, KeyState::Released),
                    Button::Controller(controller_button) => self.minput.set_controller_button(controller_button, KeyState::Released),
                }
            }
            if let Some(args) = e.update_args() {
                if let Some(tick_layered) = self.world.take_tick_layered() {
                    for layer in tick_layered.get_active_layers() {
                        for entity_id in tick_layered.get_layer(layer).expect("Tick layer was none").iter() {
                            if let Some(mut entity) = self.world.take_entity_by_id(*entity_id) {
                                entity.tick(args.dt, manager, &mut self.world, &self.minput);
                                self.world.give_entity(entity);
                            }
                        }
                    }
                    self.world.give_tick_layered(tick_layered);
                }
            }
            e.mouse_cursor(|x, y| {
                self.minput.set_cursor([x, y]);
            });
        }
    }
}
