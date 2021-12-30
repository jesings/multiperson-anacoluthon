use std::sync::{*, mpsc::*};

use crate::gamestate::Gamedata;
use crate::net::{pkt::PktPayload, *};
use crate::player::player::*;
use crate::map::grid::*;

pub fn serveloop((stream, addr): (std::net::TcpStream, std::net::SocketAddr), gd: Arc<Gamedata>, sender: mpsc::Sender<PktPayload>, mut br: bus::BusReader<PktPayload>) -> Result<(), String> {
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

    //uhh what about more than this?
    let gd = Arc::new(Gamedata {
        players: vec![Arc::new(Mutex::new(Player::test_player(0)))],
        grid: Grid::gen_blank_grid(480, 640),
    });

    servnet::launch_server_workers(streams, gd.clone(), mpsc_tx, &mut spmc);

    //now transmit gamedata

    loop {
    }
}
