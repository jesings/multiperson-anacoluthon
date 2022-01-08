use crate::gamestate::*;
use super::keyboard::*;
use super::actions::Action;

use std::sync::*;
use std::time::Duration;
use sdl2::*;

const KEYHOLDDELAY: Duration = Duration::from_millis(30);

#[derive(PartialEq, Clone, Copy)]
enum Keystate {
    None,
    Press(Duration),
    Tap,
    Hold
}

pub struct Controller {
    // what happens when you pour orange juice into milk?
    k: Keyboard,
    u: Keystate,
    r: Keystate,
    d: Keystate,
    l: Keystate,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            k: Keyboard::default(),
            u: Keystate::None,
            d: Keystate::None,
            l: Keystate::None,
            r: Keystate::None,
        }
    }
    pub fn control(&mut self, pump: &Mutex<EventPump>, gametime: Duration, gamedata: Arc<Gamedata>, pid: usize) -> bool {
        let mut callstack = vec![];
        
        for event in pump.lock().unwrap().poll_iter() {
            match event {
                event::Event::Quit {..} => {
                    return false
                },
                event::Event::KeyDown {keycode: Some(keyp), repeat: false, .. } => {
                    match self.k.convert(keyp) {
                        Action::Up => {
                            self.u = Keystate::Press(gametime);
                        },
                        Action::Down => {
                            self.d = Keystate::Press(gametime);
                        },
                        Action::Left => {
                            self.l = Keystate::Press(gametime);
                        },
                        Action::Right => {
                            self.r = Keystate::Press(gametime);
                        },
                        _ => {}
                    }
                },
                event::Event::KeyUp {keycode: Some(keyp), repeat: false, .. } => {
                    match self.k.convert(keyp) {
                        Action::Up => {
                            self.u = if self.u == Keystate::Hold {Keystate::None} else {Keystate::Tap};
                        },
                        Action::Down => {
                            self.d = if self.d == Keystate::Hold {Keystate::None} else {Keystate::Tap};
                        },
                        Action::Left => {
                            self.l = if self.l == Keystate::Hold {Keystate::None} else {Keystate::Tap};
                        },
                        Action::Right => {
                            self.r = if self.r == Keystate::Hold {Keystate::None} else {Keystate::Tap};
                        },
                        _ => {}
                    }
                },
                _ => {}
            };
        }

        let pth = |ks: &mut Keystate| {
            if let Keystate::Press(tim) = *ks {
                if gametime > tim + KEYHOLDDELAY {
                    *ks = Keystate::Hold;
                }
            }
        };
        
        pth(&mut self.u);
        pth(&mut self.d);
        pth(&mut self.l);
        pth(&mut self.r);
          
        let dir = match (self.u, self.d, self.l, self.r) {
            (Keystate::Hold | Keystate::Tap, Keystate::None, Keystate::None, Keystate::None) => Some((0, -1)),
            (Keystate::None, Keystate::Hold | Keystate::Tap, Keystate::None, Keystate::None) => Some((0, 1)),
            (Keystate::None, Keystate::None, Keystate::Hold | Keystate::Tap, Keystate::None) => Some((-1, 0)),
            (Keystate::None, Keystate::None, Keystate::None, Keystate::Hold | Keystate::Tap) => Some((1, 0)),
            
            (Keystate::Press(_) | Keystate::Hold | Keystate::Tap, Keystate::None, Keystate::Press(_) | Keystate::Hold | Keystate::Tap, Keystate::None) => Some((-1, -1)),
            (Keystate::Press(_) | Keystate::Hold | Keystate::Tap, Keystate::None, Keystate::None, Keystate::Press(_) | Keystate::Hold | Keystate::Tap) => Some((1, -1)),
            (Keystate::None, Keystate::Press(_) | Keystate::Hold | Keystate::Tap, Keystate::Press(_) | Keystate::Hold | Keystate::Tap, Keystate::None) => Some((-1, 1)),
            (Keystate::None, Keystate::Press(_) | Keystate::Hold | Keystate::Tap, Keystate::None, Keystate::Press(_) | Keystate::Hold | Keystate::Tap) => Some((1, 1)),

            _ => None,
        };
        
        let ttn = |ks: &mut Keystate| {
            if *ks == Keystate::Tap {
                *ks = Keystate::None;
            }
        };
        ttn(&mut self.u);
        ttn(&mut self.d);
        ttn(&mut self.l);
        ttn(&mut self.r);
        
        
        match dir {
            Some(dir) => {
                callstack.push(gamedata.players[pid].lock().unwrap().class.mov(pid, dir, gametime));
                
                let pth = |ks: &mut Keystate| {
                    if let Keystate::Press(_) = *ks {
                        *ks = Keystate::Hold;
                    }
                };
                
                pth(&mut self.u);
                pth(&mut self.d);
                pth(&mut self.l);
                pth(&mut self.r);
                
            },
            None => {},
        }
        
        for callback in callstack {
            (callback)(gamedata.clone());
        }
        true
    }
}
