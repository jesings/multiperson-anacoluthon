use std::net::*;
use std::sync::*;

use crate::gamestate::{Gamedata};
use crate::net::pkt;
use crate::net::pkt::PktPayload;

pub fn netloop(mut stream: TcpStream, gamedata: Arc<Gamedata>, pid: usize, rsbc: Arc<atomic::AtomicBool>) -> Result<(), String> {
    loop {
        match pkt::recv_pkt(&mut stream) {
            Ok(recvd) => {
                match recvd {
                    PktPayload::Gamedata(_) => {
                        unreachable!();
                    }
                    PktPayload::Delta(deltavec) => {
                        for delta in deltavec {
                            if delta.pid == pid {
                                let mut deltaplayer = gamedata.players[delta.pid].lock().unwrap();
                                deltaplayer.pos.0 += delta.poschange.0;
                                deltaplayer.pos.1 += delta.poschange.1;
                            }
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

        if !rsbc.load(atomic::Ordering::Relaxed) {
            break;
        }
    }
    return Ok(());
}
