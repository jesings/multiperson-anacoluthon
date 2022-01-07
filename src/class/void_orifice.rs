use serde::{Serialize, Deserialize};
use std::sync::*;
use std::time::Duration;

use super::class::*;

use crate::map::grid::*;
use crate::gamestate::*;
use crate::net::pkt::PktPayload;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct VoidOrifice {
    mov_cd: Duration,
    mov_next: Duration,
}

impl VoidOrifice {
    pub fn new() -> Self {
        VoidOrifice {
            mov_cd: Duration::from_millis(500),
            mov_next: Duration::from_millis(0),
        }
    }
    pub fn mov(&mut self, pid: usize, dir: (isize, isize), now: Duration) -> impl Fn(Arc<Gamedata>, &mpsc::Sender<PktPayload>) -> () {
        let drc = match dir {
            (0, c) => (0, c),
            (r, _) => (r, 0),
        };
        let offcd = self.mov_next < now;
        if offcd {
            self.mov_next = now + self.mov_cd;
        }
        move |gamedata: Arc<Gamedata>, sender: &mpsc::Sender<PktPayload>| {
            if offcd {
                let pl = &mut gamedata.players[pid].lock().unwrap();
                // v-w-y <- wubbles the funny collision check
                pl.pos = (pl.pos.0 + drc.0, pl.pos.1 + drc.1);
                sender.send(PktPayload::Delta(vec!(DeltaEvent{pid: pid, poschange: drc}))).unwrap();
            }
        }
    }

}
