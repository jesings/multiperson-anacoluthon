use std::net::*;
use std::io::Read;

use crate::gamestate::GDTuple;

pub struct ServerNetstate {
    pub streams: Vec<(TcpStream, SocketAddr)>,
}

pub struct ClientNetstate {
    pub stream: TcpStream,
}


pub enum PktResult {
    Gamedata(GDTuple), //initial, available on request
    Delta, //should return some delta structure
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum PktType {
    FullGamedata, //initial, available on request
    Delta,
}

//this is all that is necessary
#[repr(packed(1))]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PktHeader {
    pub tag: PktType,
    pub payload_len: usize
}

pub fn recv_pkt(stream: &mut TcpStream) -> Result<PktResult, String> {
    let mut headerbuf = [0; std::mem::size_of::<PktHeader>()];
    if let Ok(num) = stream.read(&mut headerbuf) {
        if num < std::mem::size_of::<PktHeader>() {
            return Err("Packet header was malformed".to_string());
        }
    } else {
        return Err("No packet found".to_string());
    }

    let header: PktHeader;
    unsafe {
        header = *((&headerbuf) as *const u8).cast::<PktHeader>();
    }

    let mut payloadbuf = Vec::with_capacity(header.payload_len);
    unsafe {
        payloadbuf.set_len(header.payload_len);
    }

    if let Err(_) = stream.read(payloadbuf.as_mut_slice()) {
        return Err("Malformed packet payload".to_string());
    }

    //deserialize payload
    match header.tag {
        PktType::FullGamedata => {
            let payload: GDTuple = bincode::deserialize(payloadbuf.as_slice()).unwrap();
            return Ok(PktResult::Gamedata(payload));
        }
        PktType::Delta => {
            return Ok(PktResult::Delta);
        }
    }
}
