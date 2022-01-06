use std::sync::{*, mpsc::*};

use crate::gamestate::{Gamedata, GDTuple};
use crate::net::{pkt::PktPayload, *};
use crate::player::player::*;
use crate::map::grid::*;

pub fn serveloop((mut stream, addr): (std::net::TcpStream, std::net::SocketAddr), gd: Arc<Gamedata>, sender: mpsc::Sender<PktPayload>, mut br: bus::BusReader<Arc<PktPayload>>, runningstate: Arc<atomic::AtomicBool>, index: usize) -> Result<(), String> {

    pkt::send_pkt(&mut stream, Arc::new(PktPayload::Gamedata(GDTuple {0: gd.players.iter().map(|x| (*x.lock().unwrap()).clone()).collect(), 1: 0i128, 2: index}))).expect("Could not send initialization packet");

    loop {
        while let Ok(recvd) = pkt::recv_pkt(&mut stream) {
            sender.send(recvd).unwrap();
        }
        //if this doesn't run, assume for now it's just because we're nonblocking

        while let Ok(recvd) = br.try_recv() {
            if let Err(s) = pkt::send_pkt(&mut stream, recvd) {
                println!("{} for address {}", s, addr);
            }
        }

        if !runningstate.load(atomic::Ordering::Relaxed) {
            break;
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 1000));
    }

    return Ok(());
}

pub fn gameloop() {
    let streams = servnet::initialize_server("127.0.0.1:9495".to_string());
    let mut spmc = bus::Bus::new(2048);

    let runningstate = Arc::new(atomic::AtomicBool::new(true));

    let (mpsc_tx, mpsc_rx) = channel();

    let seed = rand::random::<i128>();
    let mut playarrs = vec!();
    for i in 0..streams.len() {
        playarrs.push(Player::test_player(i));
    }

    let gd = Arc::new(Gamedata {
        players: playarrs.drain(..).map(|x| Arc::new(Mutex::new(x))).collect(),
        grid: Grid::gen_blank_grid(640, 480),
    });
    let handles = servnet::launch_server_workers(streams, gd.clone(), mpsc_tx, runningstate.clone(), &mut spmc);

    //figure out how to kill gracefully
    loop {
        while let Ok(recvd) = mpsc_rx.try_recv() {
            if let PktPayload::Delta(deltalist) = recvd {
                for delta in deltalist {
                    let mut deltaplayer = gd.players[delta.pid].lock().unwrap();
                    deltaplayer.pos.0 += delta.poschange.0;
                    deltaplayer.pos.1 += delta.poschange.1;
                    //check that this position is valid, if not revert!?
                }
            }
        }

        if Arc::strong_count(runningstate) == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 1000));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
