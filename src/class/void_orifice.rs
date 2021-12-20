use std::sync::*;
use super::class::*;

use crate::map::grid::*;
use crate::gamestate::*;


pub struct VoidOrifice {
    // ._. <- bob the funny chungus
}

impl Classlike for VoidOrifice {
    fn mov(&mut self, pid: usize, dir: (usize, usize)) -> impl Fn() {
        let drc = match dir {
            (0, c) => (0, c),
            (r, _) => (r, 0),
        };
        move |gamedata: Arc<Gamedata>| {
            let mut pl = gamedata.players[pid];
            // v-w-y <- wubbles the funny collision check
            pl.pos = (pl.pos.0 + drc.0, pl.pos.1 + drc.1);
        }
    }
}
