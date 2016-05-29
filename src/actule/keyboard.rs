
use piston_window::*;
use std::collections::HashMap;



const ESC: Button = Button::Keyboard(Key::Escape);
#[derive(Debug, PartialEq)]
pub enum KeyState {
    Pressed,
    Released
}

#[derive(Debug)]
pub struct Keyboard {
    state: HashMap<Button, KeyState>
}

impl Keyboard {
    pub fn new() -> Keyboard {
        //should Keyboard start with some keys like ESC and arrow keys already inserted?
        let mut start_state = HashMap::new();
        start_state.insert(ESC, KeyState::Released);
        Keyboard {
            state: start_state
        }
    }
    pub fn new_key(&mut self, new_key: Button) {
        self.state.insert(new_key, KeyState::Released);
        println!("added a new key {:?}", new_key);
    }
    pub fn set_key(&mut self, key: Button, key_state: KeyState) {
        *self.state.get_mut(&key).unwrap() = key_state;

    }
    pub fn read_keystate(&self, key: Button) -> &KeyState {
        self.state.get(&key).expect("key does not exist")
    }
    pub fn take_input(&mut self, input: Input) {
        match input {
            //could probably write a macro for this
            Input::Press(ESC) => {
                if self.read_keystate(ESC) != &KeyState::Pressed {
                    self.set_key(ESC, KeyState::Pressed)
                }
                else {}
            }
            Input::Release(ESC) => {
                if self.read_keystate(ESC) != &KeyState::Pressed {
                    self.set_key(ESC, KeyState::Pressed)
                }
                else {}
            }
            _ => ()
        }
    }
}
