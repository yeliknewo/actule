
use piston_window::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyState {
    Pressed,
    Released
}

#[derive(Debug)]
pub struct Keyboard {
    state: HashMap<Key, KeyState>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            state: HashMap::new()
        }
    }

    pub fn set_key(&mut self, key: Key, key_state: KeyState) {
        self.state.insert(key, key_state);

    }

    pub fn read_keystate(&self, key: Key) -> Option<&KeyState> {
        self.state.get(&key)
    }

    pub fn get_key_state(&self, key: Key) -> KeyState {
        if let Some(state) = self.state.get(&key) {
            *state
        } else {
            KeyState::Released
        }
    }
}
