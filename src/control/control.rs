use crate::gamestate::*;
use super::keyboard::*;
use super::actions::Action;

use std::sync::*;
use std::time::Duration;
use sdl2::*;


pub struct Controller {
    k: Keyboard
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            k: Keyboard::default(),
        }
    }
    pub fn control(&mut self, pump: &Mutex<EventPump>, now: Duration, gamedata: Arc<Gamedata>, pid: usize) -> bool {
        let mut callstack = vec![];
        
        for event in pump.lock().unwrap().poll_iter() {
            match event {
                event::Event::Quit {..} => {
                    return false
                },
                event::Event::KeyDown {keycode: Some(keyp), repeat: false, .. } => {
                    match self.k.convert(keyp) {
                        Action::Up => {
                            callstack.push(gamedata.players[pid].lock().unwrap().class.mov(pid, (0, -1), now));
                        },
                        Action::Down => {
                            callstack.push(gamedata.players[pid].lock().unwrap().class.mov(pid, (0, 1), now));
                        },
                        Action::Left => {
                            callstack.push(gamedata.players[pid].lock().unwrap().class.mov(pid, (-1, 0), now));
                        },
                        Action::Right => {
                            callstack.push(gamedata.players[pid].lock().unwrap().class.mov(pid, (1, 0), now));
                        },
                        _ => {}
                    }
                },
                _ => {}
            };
        }

        for callback in callstack {
            (callback)(gamedata.clone());
        }
        true
    }
}
