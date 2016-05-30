
use piston_window::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyState {
    Pressed,
    Released
}

#[derive(Debug)]
pub struct Minput {
    state: HashMap<Key, KeyState>,
    cursor: [f64; 2]
}

impl Minput {
    pub fn new() -> Minput {

        Minput {
            state: HashMap::new(),
            cursor: [0.0; 2]
        }
    }

    pub fn set_cursor(&mut self, pos: [f64; 2]) {
        self.cursor = pos;
        println!("cursor set to {:?}", pos);
    }

    pub fn set_key(&mut self, key: Key, key_state: KeyState) {
        self.state.insert(key, key_state);
        println!("set {:?} to {:?}", key, key_state);

    }

    pub fn get_key_state(&self, key: Key) -> KeyState {
        if let Some(state) = self.state.get(&key) {
            *state
        } else {
            KeyState::Released
        }
    }
}
