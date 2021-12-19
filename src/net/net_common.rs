use std::net::*;

pub struct ServerNetstate {
    pub listener: TcpListener,
    pub streams: Vec<TcpStream>,
}

pub struct ClientNetstate {
    pub stream: TcpStream,
}
