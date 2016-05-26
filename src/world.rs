use std::hash::Hash;
use std::collections::{HashMap, HashSet};

use id_alloc::*;

use entity::{Entity};
use utils::{Layer};

#[derive(Debug)]
pub struct World<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> {
    entities: HashMap<I, T>,
    names: HashMap<&'static str, I>,
    tick_ids: Option<HashSet<I>>,
    render_ids: HashMap<Layer, HashSet<I>>,
    active_layers: Vec<Layer>,
}

impl<I: Num + Bounded + Ord + CheckedAdd + CheckedSub + One + Copy + Hash, T: Entity<I, T>> World<I, T> {
    #[inline]
    pub fn new() -> World<I, T> {
        World {
            entities: HashMap::new(),
            names: HashMap::new(),
            tick_ids: Some(HashSet::new()),
            render_ids: HashMap::new(),
            active_layers: vec!(),
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
    pub fn get_active_layers(&self) -> &Vec<Layer> {
        &self.active_layers
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
        let mut layer = 0;

        if let Some(entity) = self.get_entity_by_id(id) {
            tick = entity.is_tick();
            if let Some(renderable) = entity.get_renderable() {
                render = true;
                layer = renderable.get_layer();
            }
        }

        if tick {
            self.tick_ids.as_mut().expect("Tick ids was none in add event Ids").insert(id);
        }

        if render {
            if self.render_ids.contains_key(&layer) {
                self.render_ids.get_mut(&layer).expect("Render ids was none somehow").insert(id);
            } else {
                let mut layer_set = HashSet::new();
                layer_set.insert(id);
                self.render_ids.insert(layer, layer_set);
            }
            self.active_layers.push(layer);
            self.active_layers.sort();
            self.active_layers.dedup();
        }
    }

    fn remove_event_ids(&mut self, id: I) {
        self.tick_ids.as_mut().expect("Tick ids was none in remove event ids").remove(&id);
        let mut layer = 0;
        if let Some(entity) = self.get_entity_by_id(id) {
            if let Some(renderable) = entity.get_renderable() {
                layer = renderable.get_layer();
            }
        }
        if let Some(layer_set) = self.render_ids.get_mut(&layer) {
            layer_set.remove(&id);
        }
    }

    #[inline]
    pub fn update_event_ids_by_id(&mut self, id: I) {
        self.remove_event_ids(id);
        self.add_event_ids(id);
    }

    #[inline]
    pub fn get_render_ids(&self) -> &HashMap<Layer, HashSet<I>> {
        &self.render_ids
    }

    #[inline]
    pub fn take_tick_ids(&mut self) -> Option<HashSet<I>> {
        self.tick_ids.take()
    }

    #[inline]
    pub fn give_tick_ids(&mut self, tick_ids: HashSet<I>) {
        self.tick_ids = Some(tick_ids);
    }
}
