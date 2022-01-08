use std::sync::{*, mpsc::*};
use std::collections::VecDeque;

use crate::gamestate::{Gamedata, GDTuple};
use crate::net::{pkt::PktPayload, *};
use crate::player::player::*;
use crate::map::grid::*;

const NET_HZ: u32 = 1000;

pub fn serveloop((mut stream, addr): (std::net::TcpStream, std::net::SocketAddr), gd: Arc<Gamedata>, sender: mpsc::Sender<PktPayload>, mut br: bus::BusReader<Arc<PktPayload>>, livelisteners: Arc<atomic::AtomicUsize>, index: usize) -> Result<(), String> {
    if let Ok(recvd) = br.recv() {
        let newpkt;
        if let PktPayload::Gamedata(gdt) = (*recvd).clone() {
            let mut newgdt = gdt;
            newgdt.2 = index;
            newpkt = PktPayload::Gamedata(newgdt);
        } else {
            panic!("Initialization packet was not a gamedata send?");
        }
        if let Err(s) = pkt::send_pkt(&mut stream, Arc::new(newpkt)) {
            if s.as_str() == "Fatal" {
                livelisteners.fetch_sub(1, atomic::Ordering::Relaxed);
                return Ok(());
            }
        }
    } else {
        panic!("Initialization send failed");
    }

    loop {
        let mut killflag = false;
        loop {
            match pkt::recv_pkt(&mut stream)  {
                Ok(recvd) => {
                    sender.send(recvd).unwrap();
                }
                Err(s) => {
                    if s.as_str() == "Fatal" {
                        livelisteners.fetch_sub(1, atomic::Ordering::Relaxed);
                        killflag = true;
                    }
                    break;
                }
            }
        }
        if killflag || livelisteners.load(atomic::Ordering::Relaxed) == 0 {
            break;
        }
        //if this doesn't run, assume for now it's just because we're nonblocking

        while let Ok(recvd) = br.try_recv() {
            if let Err(s) = pkt::send_pkt(&mut stream, recvd) {
                if s.as_str() == "Fatal" {
                    livelisteners.fetch_sub(1, atomic::Ordering::Relaxed);
                    killflag = true;
                }
            }
        }

        if killflag || livelisteners.load(atomic::Ordering::Relaxed) == 0 {
            break;
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / NET_HZ));
    }

    return Ok(());
}

pub fn gameloop() {
    let streams = servnet::initialize_server("127.0.0.1:9495".to_string());
    let mut spmc = bus::Bus::new(2048);

    let livelisteners = Arc::new(atomic::AtomicUsize::new(streams.len()));

    let (mpsc_tx, mpsc_rx) = channel();

    let mut mapseed = [0u8; 32];
    for i in 0..32 {
        mapseed[i] = rand::random::<u8>();
    }

    let mut playarrs = vec!();
    for i in 0..streams.len() {
        playarrs.push(Player::test_player(i));
    }

    let gd = Arc::new(Gamedata {
        players: playarrs.drain(..).map(|x| Arc::new(Mutex::new(x))).collect(),
        grid: Grid::gen_cell_auto(640, 480, mapseed),
    });


    let handles = servnet::launch_server_workers(streams, gd.clone(), mpsc_tx, &mut spmc, livelisteners.clone());

    spmc.broadcast(Arc::new(PktPayload::Gamedata(GDTuple {0: gd.players.iter().map(|x| (*x.lock().unwrap()).clone()).collect(), 1: mapseed, 2: 343})));

    //figure out how to kill gracefully
    let mut broadcasts_needed = VecDeque::new();
    loop {
        while let Ok(recvd) = mpsc_rx.try_recv() {
            if let PktPayload::Delta(ref deltalist) = recvd {
                for delta in deltalist {
                    let mut deltaplayer = gd.players[delta.pid].lock().unwrap();
                    deltaplayer.pos.0 += delta.poschange.0;
                    deltaplayer.pos.1 += delta.poschange.1;
                    //check that this position is valid, if not revert!?
                }
            }
            broadcasts_needed.push_back(recvd);
        }

        while broadcasts_needed.len() > 0 {
            let frontel = broadcasts_needed.pop_front().unwrap();
            let tryfront = Arc::new(frontel);
            if let Err(_) = spmc.try_broadcast(tryfront.clone()) {
                broadcasts_needed.push_front(Arc::try_unwrap(tryfront).unwrap());
                break;
            }
        }

        if livelisteners.load(atomic::Ordering::Relaxed) == 0 {
            break;
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / NET_HZ));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
