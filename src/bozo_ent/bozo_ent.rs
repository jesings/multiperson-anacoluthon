#[derive(Debug)]
pub struct BozoEnt {
    pub bid: usize,
    pub pos: (isize, isize),
    pub bozo_kind: BozoKind,
    pub tick_next: Duration,
}

#[derive(Debug)]
pub enum BozoKind {
    Sword(Sword),
}

impl BozoKind {
    
}


impl Entity for BozoEnt {
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
