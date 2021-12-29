use std::net::*;

pub use super::net_common::ClientNetstate;

impl ClientNetstate {
    pub fn intialize_client(server_ip_str: String) -> Self {
        let client_stream = TcpStream::connect(server_ip_str).expect("Client was passed an invalid server address/it couldn't connect!");
        Self {
            stream: client_stream
        }
    }
}
