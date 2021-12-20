use std::sync::*;
use crate::gamestate::*;
use super::void_orifice::*;

pub enum Class {
    VoidOrifice(VoidOrifice)
}

impl Class {
    fn mov(&mut self, pid: usize, dir: (usize, usize)) -> impl Fn(Arc<Gamedata>) -> () {
        match self {
            Class::VoidOrifice(void_orifice) => void_orifice.mov(pid, dir),
        }
    }
    
    pub fn void_orifice() -> Self {
        Class::VoidOrifice(VoidOrifice {
            // ._. <- bob the funny chungus gets inistantizieided
        })
    }
}
