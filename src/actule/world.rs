use std::hash::Hash;
use std::collections::{HashMap};

use id_alloc::*;

use actule::*;

#[derive(Debug)]
pub struct World<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> {
    entities: HashMap<I, T>,
    names: HashMap<&'static str, I>,
    tick_layers: Option<Layered<Layer, I>>,
    render_layers: Layered<Layer, I>,
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> World<I, T> {
    #[inline]
    pub fn new() -> World<I, T> {
        World {
            entities: HashMap::new(),
            names: HashMap::new(),
            tick_layers: Some(Layered::new()),
            render_layers: Layered::new(),
        }
    }

    #[inline]
    pub fn register_name(&mut self, id: I, name: &'static str) {
        if !self.names.contains_key(name) {
            self.names.insert(name, id);
        } else {
            panic!("Name already in use: {}", name);
        }
    }

    #[inline]
    pub fn deregister_name(&mut self, name: &'static str) {
        self.names.remove(name);
    }

    #[inline]
    pub fn get_entity_by_name(&self, name: &'static str) -> Option<&T> {
        self.get_entity_by_id(*self.names.get(name).expect("Name was not found"))
    }

    #[inline]
    pub fn get_mut_entity_by_name(&mut self, name: &'static str) -> Option<&mut T> {
        let id = *self.names.get(name).expect("Name was not found");
        self.get_mut_entity_by_id(id)
    }

    #[inline]
    pub fn get_entity_by_id(&self, id: I) -> Option<&T> {
        self.entities.get(&id)
    }

    #[inline]
    pub fn get_mut_entity_by_id(&mut self, id: I) -> Option<&mut T> {
        self.entities.get_mut(&id)
    }

    #[inline]
    pub fn add_entity(&mut self, entity: T) {
        let id = entity.get_id();
        self.entities.insert(id, entity);
        self.add_event_ids(id);
    }

    #[inline]
    pub fn take_entity_by_id(&mut self, id: I) -> Option<T> {
        self.entities.remove(&id)
    }

    #[inline]
    pub fn take_entity_by_name(&mut self, name: &'static str) -> Option<T> {
        let id = *self.names.get(name).expect("Name was not found");
        self.take_entity_by_id(id)
    }

    #[inline]
    pub fn give_entity(&mut self, entity: T) {
        self.entities.insert(entity.get_id(), entity);
    }

    #[inline]
    pub fn get_entitites(&self) -> &HashMap<I, T> {
        &self.entities
    }

    #[inline]
    pub fn get_mut_entities(&mut self) -> &mut HashMap<I, T> {
        &mut self.entities
    }

    fn add_event_ids(&mut self, id: I) {
        let mut tick = false;
        let mut render = false;

        let mut tick_layer = 0;
        let mut render_layer = 0;

        if let Some(entity) = self.get_entity_by_id(id) {
            tick = entity.is_tick();
            tick_layer = entity.get_tick_layer();
            if let Some(renderable) = entity.get_renderable() {
                render = true;
                render_layer = renderable.get_layer();
            }
        }

        if tick {
            self.tick_layers.as_mut().expect("Tick Layers was none").add(tick_layer, id);
        }

        if render {
            self.render_layers.add(render_layer, id);
        }
    }

    fn remove_event_ids(&mut self, id: I) {
        let mut tick_layer = None;
        let mut render_layer = None;
        if let Some(entity) = self.get_entity_by_id(id) {
            tick_layer = Some(entity.get_tick_layer());
            if let Some(renderable) = entity.get_renderable() {
                render_layer = Some(renderable.get_layer());
            }
        }
        if let Some(layer) = tick_layer {
            self.tick_layers.as_mut().expect("Tick layers was none").remove(layer, id);
        }
        if let Some(layer) = render_layer {
            self.render_layers.remove(layer, id);
        }
    }

    #[inline]
    pub fn update_event_ids_by_id(&mut self, id: I) {
        self.remove_event_ids(id);
        self.add_event_ids(id);
    }

    #[inline]
    pub fn get_tick_layered(&self) -> Option<&Layered<Layer, I>> {
        self.tick_layers.as_ref()
    }

    #[inline]
    pub fn get_render_layered(&self) -> &Layered<Layer, I> {
        &self.render_layers
    }

    #[inline]
    pub fn get_mut_tick_layered(&mut self) -> Option<&mut Layered<Layer, I>> {
        self.tick_layers.as_mut()
    }

    #[inline]
    pub fn get_mut_render_layered(&mut self) -> &mut Layered<Layer, I> {
        &mut self.render_layers
    }

    #[inline]
    pub fn take_tick_layered(&mut self) -> Option<Layered<Layer, I>> {
        self.tick_layers.take()
    }

    #[inline]
    pub fn give_tick_layered(&mut self, tick_layered: Layered<Layer, I>) {
        self.tick_layers = Some(tick_layered);
    }
}
