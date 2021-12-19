use std::net::*;
use super::net_common::*;

impl ClientNetstate {
    pub fn intialize_client(server_ip_str: String) -> Self {
        let client_stream = TcpStream::connect(server_ip_str).expect("Client was passed an invalid server address/it couldn't connect!");
        return Self {
            stream: client_stream
        };
    }
}
