use std::sync::*;
use crate::gamestate::*;
use super::void_orifice::*;

pub enum Class {
    VoidOrifice(VoidOrifice)
}

pub trait Classlike {
    fn mov(&mut self, pid: usize, dir: (usize, usize)) -> impl Fn();
}

impl Classlike for Class {
    fn mov(&mut self, pid: usize, dir: (usize, usize)) -> impl Fn() {
        match self {
            Class::VoidOrifice(void_orifice) => void_orifice.mov(pid, dir),
        };
    }
}

impl Class {
    pub fn void_orifice() -> Self {
        Class::VoidOrifice(VoidOrifice {
            // ._. <- bob the funny chungus gets inistantizieided
        })
    }
}
