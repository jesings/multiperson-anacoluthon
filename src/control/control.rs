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
                            self.u = Keystate::Press(now);
                        },
                        Action::Down => {
                            self.d = Keystate::Press(now);
                        },
                        Action::Left => {
                            self.l = Keystate::Press(now);
                        },
                        Action::Right => {
                            self.r = Keystate::Press(now);
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

        match self.u {
            Keystate::Press(tim) => {
                if now > tim + KEYHOLDDELAY {
                    self.u = Keystate::Hold;
                }
            }
            _ => {}
        }
        match self.d {
            Keystate::Press(tim) => {
                if now > tim + KEYHOLDDELAY {
                    self.d = Keystate::Hold;
                }
            }
            _ => {}
        }
        match self.l {
            Keystate::Press(tim) => {
                if now > tim + KEYHOLDDELAY {
                    self.l = Keystate::Hold;
                }
            }
            _ => {}
        }
        match self.r {
            Keystate::Press(tim) => {
                if now > tim + KEYHOLDDELAY {
                    self.r = Keystate::Hold;
                }
            }
            _ => {}
        }
        
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

        if self.u == Keystate::Tap {
            self.u = Keystate::None;
        }
        if self.d == Keystate::Tap {
            self.d = Keystate::None;
        }
        if self.l == Keystate::Tap {
            self.l = Keystate::None;
        }
        if self.r == Keystate::Tap {
            self.r = Keystate::None;
        }
        
        match dir {
            Some(dir) => {
                callstack.push(gamedata.players[pid].lock().unwrap().class.mov(pid, dir, now));
                match self.u {
                    Keystate::Press(_) => { self.u = Keystate::Hold; }
                    _ => {}
                }
                match self.d {
                    Keystate::Press(_) => { self.d = Keystate::Hold; }
                    _ => {}
                }
                match self.l {
                    Keystate::Press(_) => { self.l = Keystate::Hold; }
                    _ => {}
                }
                match self.r {
                    Keystate::Press(_) => { self.r = Keystate::Hold; }
                    _ => {}
                }
                    
            },
            None => {},
        }
        
        for callback in callstack {
            (callback)(gamedata.clone());
        }
        true
    }
}
