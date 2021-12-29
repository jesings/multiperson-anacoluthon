use std::sync::*;

use crate::gamestate::Gamedata;
use crate::net::pkt::PktPayload;

pub fn serveloop((stream, addr): (std::net::TcpStream, std::net::SocketAddr), gd: Arc<Mutex<Gamedata>>, sender: mpsc::Sender<PktPayload>, mut br: bus::BusReader<PktPayload>) -> Result<(), String> {
    let tx = sender;
    let rx = br;
    while true {
    }
    return Ok(());
}
