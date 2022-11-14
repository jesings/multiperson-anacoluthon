use std::collections::BTreeMap;
use std::time::Duration;
use std::sync::*;

use crate::gamestate::*;
use crate::net::pkt::{PktType, PktPayload};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Etype {
    Player,
    Enemy,
    BozoEnt
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
    
    fn mov(&mut self, gamedata: &Arc<Gamedata>, entid: (Etype, usize), dir: (isize, isize)) -> Option<BTreeMap<PktType, PktPayload>> {
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
        if let Some(entlist) = occupied.get_mut(&enp) {
            for ent in entlist.iter() {
                let passable = match ent {
                    (Etype::Player, pid) =>
                        self.passable(&*gamedata.players[*pid].lock().unwrap()),
                    (Etype::Enemy, eid) =>
                        self.passable(&*gamedata.enemies[*eid].lock().unwrap()),
                    (Etype::BozoEnt, bid) =>
                        self.passable(&*gamedata.bozoents.get(bid).unwrap().lock().unwrap()),
                };
                if !passable {
                    return None;
                }
            }
            let mut revert_mov = false;
            for ent in entlist.iter() {
                revert_mov = match ent {
                    (Etype::Player, pid) =>
                        gamedata.players[*pid].lock().unwrap().collide(gamedata, entid),
                    (Etype::Enemy, eid) =>
                        gamedata.enemies[*eid].lock().unwrap().collide(gamedata, entid),
                    (Etype::BozoEnt, bid) =>
                        gamedata.bozoents.get(bid).unwrap().lock().unwrap().collide(gamedata, entid),
                } | self.collide(gamedata, *ent) | revert_mov;
            }
            if revert_mov {
                return None;
            }
            entlist.push(entid);
        } else {
            occupied.insert(enp, vec![entid]);
        };
        
        if let Some(preventlist) = occupied.get_mut(&prevpos) {
            if preventlist.len() > 1 {
                let previndex = preventlist.iter().position(|&x| x == entid).unwrap();
                preventlist.remove(previndex);
            } else {
                occupied.remove(&prevpos);
            };
        };
        
        drop(occupied);
        *self.mut_pos() = enp;
        self.on_mov(gamedata, entid, prevpos); //if this modifies the player in such a way that a packet needs to be sent we may need to change this up but for now I don't care
        return Some(enp);
    }
    fn passable<T: Entity>(&self, _other: &T) -> bool {
        false
    }
    fn mov_timeout(&mut self, now: Duration) {
        *self.mut_mov_next() = now + self.move_timeout();
    }
    fn collide(&mut self, _gamedata: &Arc<Gamedata>, _other: (Etype, usize)) -> bool {
        false
    }
}
