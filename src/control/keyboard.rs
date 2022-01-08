use super::actions::Action;

use sdl2::keyboard::Keycode;

use std::collections::HashMap;

pub struct Keyboard {
    hm: HashMap<Keycode, Action>,
}

impl Keyboard {
    pub fn default() -> Self {
        Keyboard {
            hm: HashMap::from([
                (Keycode::W, Action::Up),
                (Keycode::A, Action::Left),
                (Keycode::S, Action::Down),
                (Keycode::D, Action::Right),

                (Keycode::Up, Action::Up),
                (Keycode::Left, Action::Left),
                (Keycode::Down, Action::Down),
                (Keycode::Right, Action::Right),
                
            ]),
        }
    }
    
    // read keys from file...
    // pub fn from_file() -> Self {..}
    
    pub fn convert(&self, key: Keycode) -> Action {
        match self.hm.get(&key) {
            Some(action) => *action,
            None => Action::None
        }
    }
}
