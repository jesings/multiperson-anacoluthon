use std::net::*;

pub struct ServerNetstate {
    pub streams: Vec<(TcpStream, SocketAddr)>,
}

pub struct ClientNetstate {
    pub stream: TcpStream,
}
