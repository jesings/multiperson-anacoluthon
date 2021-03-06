use std::net::*;
use std::sync::*;

use crate::gamestate::{Gamedata};
use crate::net::pkt;
use crate::net::pkt::PktPayload;
use crate::entity::entity::{Entity,Etype};

const CLIENT_NET_HZ: u32 = 1000;

pub fn netloop(mut stream: TcpStream, gamedata: Arc<Gamedata>, pid: usize, rsbc: Arc<atomic::AtomicBool>, recver: mpsc::Receiver<PktPayload>) -> Result<(), String> {
    loop {
        match pkt::recv_pkt(&mut stream) {
            Ok(recvd) => {
                match recvd {
                    PktPayload::Initial(_) => {
                        unreachable!();
                    }
                    PktPayload::PlayerDelta(deltavec) => {
                        for delta in deltavec {
                            if delta.pid != pid {
                                let mut deltaplayer = gamedata.players[delta.pid].lock().unwrap();
                                let mut occupied = gamedata.occupation.write().unwrap();
                                occupied.remove(deltaplayer.pos());
                                occupied.insert(delta.newpos, (Etype::Player, delta.pid));
                                deltaplayer.pos.0 = delta.newpos.0;
                                deltaplayer.pos.1 = delta.newpos.1;
                            }
                        }
                    }
                    PktPayload::EnemyDelta(deltavec) => {
                        for delta in deltavec {
                            let mut deltaenemy = gamedata.enemies[delta.eid].lock().unwrap();
                            let mut occupied = gamedata.occupation.write().unwrap();
                            occupied.remove(deltaenemy.pos());
                            occupied.insert(delta.newpos, (Etype::Enemy, delta.eid));
                            deltaenemy.pos.0 = delta.newpos.0;
                            deltaenemy.pos.1 = delta.newpos.1;
                        }
                    }
                }
            }
            Err(errmessage) => {
                if errmessage.as_str() == "Fatal" {
                    break;
                }
            }
        }

        while let Ok(recvd) = recver.try_recv() {
            if let Err(s) = pkt::send_pkt(&mut stream, Arc::new(recvd)) {
                if s.as_str() == "Fatal" {
                    rsbc.store(false, atomic::Ordering::Relaxed);
                } else {
                    //perhaps attempt to undo the changes client side??
                }
                break;
            }
        }



        if !rsbc.load(atomic::Ordering::Relaxed) {
            break;
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / CLIENT_NET_HZ));
    }
    return Ok(());
}
