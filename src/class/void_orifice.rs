use serde::{Serialize, Deserialize};
use std::sync::*;
use std::time::Duration;

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
            mov_cd: Duration::from_millis(300),
            mov_next: Duration::from_millis(0),
        }
    }
    pub fn mov(&mut self, pid: usize, dir: (isize, isize), now: Duration) -> Option<impl Fn(Arc<Gamedata>, &mpsc::Sender<PktPayload>) -> ()> {
        let offcd = self.mov_next < now;
        if offcd {
            self.mov_next = now + self.mov_cd;
        } else {
            return None;
        }
        Some(move |gamedata: Arc<Gamedata>, sender: &mpsc::Sender<PktPayload>| {
            if offcd {
                let pl = &mut gamedata.players[pid].lock().unwrap();
                let pnp = (pl.pos.0 + dir.0, pl.pos.1 + dir.1);
                if gamedata.grid.passable(pnp) {
                    // v-w-y <- wubbles the funny collision check :D
                    pl.pos = pnp;
                    sender.send(PktPayload::Delta(vec!(DeltaEvent{pid: pid, poschange: dir}))).unwrap();
                }
            }
        })
    }

}
