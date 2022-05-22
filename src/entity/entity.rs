use std::time::Duration;
use std::sync::*;

use crate::gamestate::*;

#[derive(Clone, Copy)]
pub enum Etype {
    Player,
    Enemy
}
 
pub trait Entity {
   fn pos(&self) -> &(isize, isize);
   fn mut_pos(&mut self) -> &mut (isize, isize);
   fn move_timeout(&self) -> Duration;
   fn mut_mov_next(&mut self) -> &mut Duration;
   fn rectify_dir(&self, _gamedata: &Arc<Gamedata>, _entid: (Etype, usize), dir: (isize, isize)) -> Option<(isize, isize)> {
       Some(dir)
   }
   fn on_mov(&mut self, _gamedata: &Arc<Gamedata>, _entid: (Etype, usize), _prevpos: (isize, isize)) {
   }
   fn mov(&mut self, gamedata: &Arc<Gamedata>, entid: (Etype, usize), dir: (isize, isize), now: Duration) -> Option<(isize, isize)> {
       let offcd = *self.mut_mov_next() < now;
       if !offcd {
           return None;
       }
       let rectified_dir = if let Some(newdir) = self.rectify_dir(gamedata, entid, dir) {
           newdir
       } else {
           return None;
       };
       let prevpos = *self.mut_pos();
       let enp = (prevpos.0 + rectified_dir.0, prevpos.1 + rectified_dir.1);

       *self.mut_mov_next() = now + self.move_timeout();
       if !gamedata.grid.passable(enp) {
           return None;
       }
       *self.mut_pos() = enp;
       self.on_mov(gamedata, entid, prevpos); //if this modifies the player in such a way that a packet needs to be sent we may need to change this up but for now I don't care
       return Some(dir);
   }
}
