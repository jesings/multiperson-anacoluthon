pub use super::net_common::ServerNetstate;
use std::net::*;
use std::time::*;

const ACCEPT_WAITTIME: u64 = 10;

impl ServerNetstate {
    pub fn initialize_server(listen_ip_str: String) -> Self {
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

        Self {
            streams: streamvec,
        }
    }
}
