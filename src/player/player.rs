use serde::{Serialize, Deserialize};
use std::time::Duration;

use crate::class::class::*;
use crate::entity::entity::Entity;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Player {
    pub pid: usize,
    pub pos: (isize, isize),
    pub class: Class,
    pub mov_next: Duration,
}

impl Player {
    pub fn test_player(pid: usize, pos: (isize, isize)) -> Self {
        Player {
            pid, pos,
            class: Class::void_orifice(),
            mov_next: Duration::from_millis(0),
        }
    }
}

impl Entity for Player {
    fn pos(&self) -> &(isize, isize) {
        &self.pos
    }
    fn mut_pos(&mut self) -> &mut (isize, isize) {
        &mut self.pos
    }
    fn move_timeout(&self) -> Duration {
        self.class.move_timeout()
    }
    fn mut_mov_next(&mut self) -> &mut Duration {
        &mut self.mov_next
    }
}
