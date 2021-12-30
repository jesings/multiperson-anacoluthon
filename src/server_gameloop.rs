use std::sync::{*, mpsc::*};

use crate::gamestate::{Gamedata, GDTuple};
use crate::net::{pkt::PktPayload, *};
use crate::player::player::*;
use crate::map::grid::*;

pub fn serveloop((stream, addr): (std::net::TcpStream, std::net::SocketAddr), gd: Arc<Gamedata>, sender: mpsc::Sender<PktPayload>, mut br: bus::BusReader<Arc<PktPayload>>) -> Result<(), String> {
    let tx = sender;
    let rx = br;

    loop {
    }

    return Ok(());
}

pub fn gameloop() {
    let streams = servnet::initialize_server("127.0.0.1:9495".to_string());
    let mut spmc = bus::Bus::new(64);

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
    }
}
