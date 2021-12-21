use std::sync::*;
use std::time::Duration;

use crate::gamestate::*;
use super::void_orifice::*;

#[derive(Debug)]
pub enum Class {
    VoidOrifice(VoidOrifice)
}

impl Class {
    pub fn mov(&mut self, pid: usize, dir: (isize, isize), now: Duration) -> impl Fn(Arc<Gamedata>) -> () {
        match self {
            Class::VoidOrifice(void_orifice) => void_orifice.mov(pid, dir, now),
        }
    }
    
    pub fn void_orifice() -> Self {
        Class::VoidOrifice(VoidOrifice::new())
    }
}
