use serde::{Serialize, Deserialize};
use std::time::Duration;
use crate::entity::entity::Entity;
use super::enemy_type::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Enemy {
    pub eid: usize,
    pub pos: (isize, isize),
    pub mov_next: Duration,
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn test_enemy(eid: usize, pos: (isize, isize)) -> Self {
        Self {
            eid, pos, 
            mov_next: Duration::from_millis(0),
            enemy_type: EnemyType::sequitur()
        }
    }
    pub fn fast_enemy(eid: usize, pos: (isize, isize)) -> Self {
        Self {
            eid, pos, 
            mov_next: Duration::from_millis(0),
            enemy_type: EnemyType::polandball()
        }
    }
}

impl Entity for Enemy {
    fn pos(&self) -> &(isize, isize) {
        &self.pos
    }
    fn mut_pos(&mut self) -> &mut (isize, isize) {
        &mut self.pos
    }
    fn move_timeout(&self) -> Duration {
        self.enemy_type.move_timeout()
    }
    fn mut_mov_next(&mut self) -> &mut Duration {
        &mut self.mov_next
    }
}
