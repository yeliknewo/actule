
use piston_window::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyState {
    Pressed,
    Released
}

#[derive(Debug)]
pub struct Minput {
    keys: HashMap<Key, KeyState>,
    mouse: HashMap<MouseButton, KeyState>,
    controller: HashMap<ControllerButton, KeyState>,
    cursor: [f64; 2]
}

impl Minput {
    #[inline]
    pub fn new() -> Minput {
        Minput {
            keys: HashMap::new(),
            mouse: HashMap::new(),
            controller: HashMap::new(),
            cursor: [0.0; 2]
        }
    }

    #[inline]
    pub fn get_key(&self, key: Key) -> KeyState {
        if let Some(state) = self.keys.get(&key) {
            *state
        } else {
            KeyState::Released
        }
    }

    #[inline]
    pub fn get_mouse_button(&self, mouse_button: MouseButton) -> KeyState {
        if let Some(state) = self.mouse.get(&mouse_button) {
            *state
        } else {
            KeyState::Released
        }
    }

    #[inline]
    pub fn get_controller_button(&self, controller_button: ControllerButton) -> KeyState {
        if let Some(state) = self.controller.get(&controller_button) {
            *state
        } else {
            KeyState::Released
        }
    }

    #[inline]
    pub fn get_cursor(&self) -> [f64; 2] {
        self.cursor
    }

    #[inline]
    pub fn set_key(&mut self, key: Key, key_state: KeyState) {
        self.keys.insert(key, key_state);
        // println!("set {:?} to {:?}", key, key_state);
    }

    #[inline]
    pub fn set_mouse_button(&mut self, mouse_button: MouseButton, key_state: KeyState) {
        self.mouse.insert(mouse_button, key_state);
    }

    #[inline]
    pub fn set_controller_button(&mut self, controller_button: ControllerButton, key_state: KeyState) {
        self.controller.insert(controller_button, key_state);
    }

    #[inline]
    pub fn set_cursor(&mut self, pos: [f64; 2]) {
        self.cursor = pos;
        // println!("cursor set to {:?}", pos);
    }
}
