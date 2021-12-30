use serde::{Serialize, Deserialize};

use crate::class::class::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Player {
    pub pid: usize,
    pub pos: (isize, isize),
    pub class: Class,
}

impl Player {
    pub fn test_player(pid: usize) -> Self {
        Player {
            pid: pid,
            pos: (64, 64),
            class: Class::void_orifice(),
        }
    }
}
