use std::net::*;
use std::time::*;
use std::sync::*;
use std::thread;

pub use super::pkt::PktPayload;
use crate::gamestate::Gamedata;
use crate::server_gameloop::serveloop;

const ACCEPT_WAITTIME: u64 = 1;

pub fn initialize_server(listen_ip_str: String) -> Vec<(TcpStream, SocketAddr)> {
    let server_listener = TcpListener::bind(listen_ip_str).expect("Server was configured to bind to an invalid address!");
    let mut streamvec = vec!();

    server_listener.set_nonblocking(true).expect("Could not set listening socket to be nonblocking!");

    //do some accepts to create the streams
    let loopstarttime = Instant::now();
    while Instant::now().duration_since(loopstarttime).as_secs() < ACCEPT_WAITTIME {
        if let Ok(tcpl) = server_listener.accept() {
            streamvec.push(tcpl);
        }
    }

    streamvec
}

//consumes the self object
pub fn launch_server_workers(mut strms: Vec<(TcpStream, SocketAddr)>, gd: Arc<Gamedata>, sender: mpsc::Sender<PktPayload>, bus: &mut bus::Bus<PktPayload>) -> Vec<thread::JoinHandle<Result<(), String>>> {
    let mut launched = vec!();
    for stream in strms.drain(..) {
        let new_gd_handle = gd.clone();
        let new_sender = sender.clone();
        let new_br = bus.add_rx();
        launched.push(thread::spawn(move || {
            serveloop(stream, new_gd_handle, new_sender, new_br)
        }));
    }
    launched
}
