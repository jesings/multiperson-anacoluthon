use std::time::Duration;
use std::sync::*;

use crate::gamestate::*;

#[derive(Clone, Copy, Debug)]
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
   fn on_mov(&mut self, _gamedata: &Arc<Gamedata>, _entid: (Etype, usize), _prevpos: (isize, isize)) {}
   fn mov(&mut self, gamedata: &Arc<Gamedata>, entid: (Etype, usize), dir: (isize, isize)) -> Option<(isize, isize)> {
       let prevpos = *self.mut_pos();
       let enp;
       if let Some(newdir) = self.rectify_dir(gamedata, entid, dir) {
           enp = (prevpos.0 + newdir.0, prevpos.1 + newdir.1);
       } else {
           unreachable!();
       }

       if !gamedata.grid.passable(enp) {
           return None;
       }
       let mut occupied = gamedata.occupation.write().unwrap();
       if occupied.contains_key(&enp) {
           return None;
       }
       occupied.remove(&prevpos);
       occupied.insert(enp, entid);
       drop(occupied);
       *self.mut_pos() = enp;
       self.on_mov(gamedata, entid, prevpos); //if this modifies the player in such a way that a packet needs to be sent we may need to change this up but for now I don't care
       return Some(enp);
   }

   fn mov_timeout(&mut self, now: Duration) {
       *self.mut_mov_next() = now + self.move_timeout();
   }
}
