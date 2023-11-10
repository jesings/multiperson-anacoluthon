use serde::{Serialize, Deserialize};
use super::void_orifice::*;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Class {
    VoidOrifice(VoidOrifice)
}

impl Class {
    pub fn move_timeout(&self) -> Duration {
        match self {
            Class::VoidOrifice(void_orifice) => void_orifice.move_timeout,
        }
    }

    pub fn skill_timeout(&self, skill: usize) -> Duration {
        match self {
            Class::VoidOrifice(ref void_orifice) => void_orifice.skill_timeout[skill],
        }
    }

    pub fn mut_skill_next(&mut self, skill: usize) -> &mut Duration {
        match self {
            Class::VoidOrifice(ref mut void_orifice) => &mut void_orifice.skill_next[skill],
        }
    }

    pub fn directional_skill(&self, skill: usize) -> bool {
        match self {
            Class::VoidOrifice(ref void_orifice) => void_orifice.directional_skill[skill],
        }
    }

    pub fn void_orifice() -> Self {
        Class::VoidOrifice(VoidOrifice::new())
    }
}
