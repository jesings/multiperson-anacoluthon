use std::sync::{*, mpsc::*};

use crate::gamestate::{Gamedata, GDTuple};
use crate::net::{pkt::PktPayload, *};
use crate::player::player::*;
use crate::map::grid::*;

pub fn serveloop((mut stream, addr): (std::net::TcpStream, std::net::SocketAddr), gd: Arc<Gamedata>, sender: mpsc::Sender<PktPayload>, mut br: bus::BusReader<Arc<PktPayload>>) -> Result<(), String> {

    loop {
        while let Ok(recvd) = pkt::recv_pkt(&mut stream) {
            sender.send(recvd).unwrap();
        }
        //if this doesn't run, assume for now it's just because we're nonblocking

        while let Ok(recvd) = br.try_recv() {
            pkt::send_pkt(&mut stream, recvd);
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 1000));
    }

    return Ok(());
}

pub fn gameloop() {
    let streams = servnet::initialize_server("127.0.0.1:9495".to_string());
    let mut spmc = bus::Bus::new(2048);

    let (mpsc_tx, mpsc_rx) = channel();

    let seed = rand::random::<i128>();
    let mut playarrs = vec!();
    for i in 0..streams.len() {
        playarrs.push(Player::test_player(i));
    }
    //now transmit gamedata
    spmc.broadcast(Arc::new(PktPayload::Gamedata(GDTuple(playarrs.clone(), seed))));

    let gd = Arc::new(Gamedata {
        players: playarrs.drain(..).map(|x| Arc::new(Mutex::new(x))).collect(),
        grid: Grid::gen_blank_grid(640, 480),
    });
    let handles = servnet::launch_server_workers(streams, gd.clone(), mpsc_tx, &mut spmc);

    loop {
        let mut differences = vec!();
        while let Ok(recvd) = mpsc_rx.try_recv() {
            if let PktPayload::Delta(deltalist) = recvd {
                for delta in deltalist {
                    differences.push(delta);
                }
            }

        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 1000));
    }
}
