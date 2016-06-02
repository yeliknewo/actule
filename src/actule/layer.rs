use std::collections::{HashMap, HashSet};
use std::hash::{Hash};

#[derive(Debug)]
pub struct Layered<T: Eq + Hash + Ord + Clone, I: Eq + Hash> {
    layers: Option<HashMap<T, HashSet<I>>>,
    active_layers: Vec<T>,
}

impl<T: Eq + Hash + Ord + Clone, I: Eq + Hash> Layered<T, I> {
    pub fn new() -> Layered<T, I> {
        Layered {
            layers: Some(HashMap::new()),
            active_layers: vec!(),
        }
    }

    pub fn add(&mut self, layer_id: T, item: I) {
        let mut layers = self.layers.take().expect("Layers was none when adding");
        if layers.contains_key(&layer_id) {
            layers.get_mut(&layer_id).expect("Layer id was somehow not in layers after being in layers").insert(item);
        } else {
            let mut layer_set = HashSet::new();
            layer_set.insert(item);
            layers.insert(layer_id.clone(), layer_set);
        }
        self.active_layers.push(layer_id);
        self.active_layers.sort();
        self.active_layers.dedup();
        self.layers = Some(layers);
    }

    pub fn remove(&mut self, layer_id: T, item: I) {
        if let Some(layer_set) = self.layers.as_mut().expect("Layers was none when removing").get_mut(&layer_id) {
            layer_set.remove(&item);
        }
    }

    pub fn get_layer(&self, layer_id: &T) -> Option<&HashSet<I>> {
        self.layers.as_ref().expect("Layers was none during get").get(&layer_id)
    }

    pub fn get_all(&self) -> Option<&HashMap<T, HashSet<I>>> {
        self.layers.as_ref()
    }

    pub fn get_active_layers(&self) -> &Vec<T> {
        &self.active_layers
    }

    pub fn take_all(&mut self) -> Option<HashMap<T, HashSet<I>>> {
        self.layers.take()
    }

    pub fn give_all(&mut self, all: HashMap<T, HashSet<I>>) {
        self.layers = Some(all);
    }
}
