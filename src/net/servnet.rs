use std::net::*;
use super::net_common::*;

impl ServerNetstate {
    pub fn initialize_server(listen_ip_str: String) -> Self {
        let server_listener = TcpListener::bind(listen_ip_str).expect("Server was configured to bind to an invalid address!");
        let streamvec = vec!();

        //do some accepts to create the streams

        return Self {
            listener: server_listener,
            streams: streamvec,
        };
    }
}
