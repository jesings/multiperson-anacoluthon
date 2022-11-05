use std::time::Duration;

use crate::entity::entity::Entity;

#[derive(Debug)]
pub struct BozoEnt {
    pub bid: usize,
    pub pos: (isize, isize),
    pub bozo_kind: BozoKind,
    pub tick_next: Duration,
}

#[derive(Debug)]
pub struct Sword {
    damage: isize,
}

#[derive(Debug)]
pub enum BozoKind {
    Sword(Sword),
}

impl BozoKind {
    pub fn tick(&mut self) -> Option<(isize, isize)> {
        match self {
            BozoKind::Sword(_) => None
            _ => Some((0, 0))
        }
    }
    pub fn on_collide(&mut self, gamedata: &Arc<Gamedata>) {
        
    }
    pub fn timeout(&self) -> Duration {
        Duration::new(0,0)
    }
}



impl Entity for BozoEnt {
    fn pos(&self) -> &(isize, isize) {
        &self.pos
    }
    fn mut_pos(&mut self) -> &mut (isize, isize) {
        &mut self.pos
    }
    fn move_timeout(&self) -> Duration {
        self.bozo_kind.timeout()
    }
    fn mut_mov_next(&mut self) -> &mut Duration {
        &mut self.tick_next
    }

}
