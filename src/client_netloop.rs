use std::net::*;
use std::sync::*;

use crate::gamestate::{Gamedata};
use crate::net::pkt;
use crate::net::pkt::PktPayload;

const CLIENT_NET_HZ: u32 = 1000;

pub fn netloop(mut stream: TcpStream, gamedata: Arc<Gamedata>, pid: usize, rsbc: Arc<atomic::AtomicBool>, recver: mpsc::Receiver<PktPayload>) -> Result<(), String> {
    loop {
        match pkt::recv_pkt(&mut stream) {
            Ok(recvd) => {
                match recvd {
                    PktPayload::Initial(_) => {
                        unreachable!();
                    }
                    PktPayload::Delta(deltavec) => {
                        for delta in deltavec {
                            if delta.pid != pid {
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
