use crate::class::class::*;

pub struct Player {
    pub pid: usize,
    pub pos: (usize, usize),
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
