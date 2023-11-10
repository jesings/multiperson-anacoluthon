use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::time::Duration;

use crate::class::class::*;
use crate::entity::entity::{Etype, Entity};
use crate::gamestate::Gamedata;

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

    pub fn directional_skill(&self, skill: usize) -> bool {
        self.class.directional_skill(skill)
    }

    pub fn mut_skill_next(&mut self, skill: usize) -> &mut Duration {
        self.class.mut_skill_next(skill)
    }

    pub fn skill_timeout(&mut self, skill: usize, now: Duration) {
        *self.mut_skill_next(skill) = now + self.class.skill_timeout(skill);
    }

    pub fn skill(&mut self, gamedata: &Arc<Gamedata>, entid: (Etype, usize), skill: usize, dir: Option<(isize, isize)>) { // probably move to impl entity eventually

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
