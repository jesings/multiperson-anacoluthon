use std::sync::*;
use crate::gamestate::*;
use super::void_orifice::*;

pub enum Class {
    VoidOrifice(VoidOrifice)
}

pub trait Classlike {
    fn mov(&self, gamestate: Arc<ClientGamestate>, dir: (usize, usize));
}

impl Classlike for Class {
    fn mov(&self, gamestate: Arc<ClientGamestate>, dir: (usize, usize)) {
        match self {
            Class::VoidOrifice(void_orifice) => void_orifice.mov(gamestate, dir),
        };
    }
}
