use std::net::*;
use std::io::{Read, IoSlice, Write};

use crate::gamestate::GDTuple;

pub enum PktPayload {
    Gamedata(GDTuple), //initial, available on request
    Delta, //should return some delta structure
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum PktType {
    FullGamedata, //initial, available on request
    Delta,
}

//this is all that is necessary
#[repr(packed(1))]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct PktHeader {
    pub tag: PktType,
    pub payload_len: usize
}

pub fn recv_pkt(stream: &mut TcpStream) -> Result<PktPayload, String> {
    let mut headerbuf = [0; std::mem::size_of::<PktHeader>()];
    if let Ok(num) = stream.read(&mut headerbuf) {
        if num == 0 {
            return Err("Socket has closed".to_string());
        } else if num < std::mem::size_of::<PktHeader>() {
            return Err("Packet header was malformed".to_string());
        }
    } else {
        return Err("No packet found".to_string());
    }

    let header: PktHeader = unsafe {
        *((&headerbuf) as *const u8).cast::<PktHeader>()
    };

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
            return Ok(PktPayload::Gamedata(payload));
        }
        PktType::Delta => {
            return Ok(PktPayload::Delta);
        }
    }
}

pub fn send_pkt(stream: &mut TcpStream, payload: PktPayload) -> Result<usize, String> {
    let header;
    let paybuf;
    match payload {
        PktPayload::Gamedata(gdt) => {
            paybuf = if let Ok(s) = bincode::serialize(&gdt) {
                s
            } else {
                return Err("Could not serialize gamedata!".to_string());
            };
            header = PktHeader {tag: PktType::FullGamedata, payload_len: paybuf.len()};
        }
        PktPayload::Delta => {
            unreachable!(); //uhhh
        }
    }
    let io_header = IoSlice::new(unsafe { 
        std::slice::from_raw_parts((&header as *const PktHeader) 
            as *const u8, std::mem::size_of::<PktHeader>())
    });
    let io_payload = IoSlice::new(paybuf.as_slice());
    if let Ok(sz) = stream.write_vectored(&[io_header, io_payload]) {
        Ok(sz)
    } else {
        Err("Could not write to stream!".to_string())
    }
}
