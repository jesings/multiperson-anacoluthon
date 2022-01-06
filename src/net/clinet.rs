use std::net::*;

pub fn initialize_client(server_ip_str: String) -> TcpStream {
    let client_stream = TcpStream::connect(server_ip_str).expect("Client was passed an invalid server address/it couldn't connect!");
    client_stream.set_nonblocking().unwrap();
    client_stream
}
