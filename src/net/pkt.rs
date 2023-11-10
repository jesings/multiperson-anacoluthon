use std::collections::BTreeMap;
use std::net::*;
use std::sync::*;
use std::io::{Read, IoSlice, Write, ErrorKind};

use crate::gamestate::InitializationData;
use crate::gamestate::{PlayerDeltaEvent, EnemyDeltaEvent};

#[derive(Debug, Clone)]
pub enum PktPayload {
    Initial(InitializationData), //initial, available on request
    PlayerDelta(Vec<PlayerDeltaEvent>), //should return some delta structure
    EnemyDelta(Vec<EnemyDeltaEvent>), //should return some delta structure
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PktType {
    InitialPkt, //initial, available on request
    PlayerDelta,
    EnemyDelta,
}

//this is all that is necessary
#[repr(packed(1))]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct PktHeader {
    pub tag: PktType,
    pub payload_len: usize
}

pub fn coalesce_pkts(into: &mut BTreeMap<PktType, PktPayload>, from: BTreeMap<PktType, PktPayload>) {
    for (k,fv) in from.into_iter() {
        subsume_pkt(into, k, fv);
    }
}

pub fn subsume_pkt(into: &mut BTreeMap<PktType, PktPayload>, k: PktType, fv: PktPayload) {
    if let Some(iv) = into.get_mut(&k) {
        match (iv, fv) {
            (&mut PktPayload::Initial(ref mut ont), PktPayload::Initial(nnt)) =>
                *ont = nnt,
            (&mut PktPayload::PlayerDelta(ref mut ilst), PktPayload::PlayerDelta(ref mut flst)) =>
                ilst.append(flst),
            (&mut PktPayload::EnemyDelta(ref mut ilst), PktPayload::EnemyDelta(ref mut flst)) =>
                ilst.append(flst),
            _ => unreachable!(),
        };
    } else {
        into.insert(k, fv);
    }
}

pub fn recv_pkt(stream: &mut TcpStream) -> Result<PktPayload, String> {
    let mut headerbuf = [0; std::mem::size_of::<PktHeader>()];
    match stream.read(&mut headerbuf) {

        Ok(num) => {
            if num == 0 {
                return Err("Fatal".to_string());
            } else if num < std::mem::size_of::<PktHeader>() {
                return Err("Packet header was malformed".to_string());
            }
        }
        Err(ref e) => {
            match e.kind() {
                ErrorKind::WouldBlock |
                ErrorKind::Interrupted => {
                    return Err("No packet available".to_string());
                }
                _ => {
                    return Err("Fatal".to_string());
                }
            }
        }
    }

    let header: PktHeader = unsafe {
        *((&headerbuf) as *const u8).cast::<PktHeader>()
    };

    let mut payloadbuf = Vec::with_capacity(header.payload_len);
    unsafe {
        payloadbuf.set_len(header.payload_len);
    }

    if let Err(_) = stream.read(payloadbuf.as_mut_slice()) {
        return Err("Fatal".to_string());
    }

    //deserialize payload
    match header.tag {
        PktType::InitialPkt => {
            let payload: InitializationData = bincode::deserialize(payloadbuf.as_slice()).unwrap();
            return Ok(PktPayload::Initial(payload));
        }
        PktType::PlayerDelta => {
            let payload: Vec<PlayerDeltaEvent> = bincode::deserialize(payloadbuf.as_slice()).unwrap();
            return Ok(PktPayload::PlayerDelta(payload));
        }
        PktType::EnemyDelta => {
            let payload: Vec<EnemyDeltaEvent> = bincode::deserialize(payloadbuf.as_slice()).unwrap();
            return Ok(PktPayload::EnemyDelta(payload));
        }
    }
}

pub fn send_pkt(stream: &mut TcpStream, payload: Arc<PktPayload>) -> Result<usize, String> {
    let header;
    let paybuf;
    match *payload {
        PktPayload::Initial(ref init) => {
            paybuf = if let Ok(s) = bincode::serialize(init) {
                s
            } else {
                return Err("Could not serialize gamedata!".to_string());
            };
            header = PktHeader {tag: PktType::InitialPkt, payload_len: paybuf.len()};
        }
        PktPayload::PlayerDelta(ref deltavec) => {
            paybuf = if let Ok(s) = bincode::serialize(deltavec) {
                s
            } else {
                return Err("Could not serialize deltas!".to_string());
            };
            header = PktHeader {tag: PktType::PlayerDelta, payload_len: paybuf.len()};
        }
        PktPayload::EnemyDelta(ref deltavec) => {
            paybuf = if let Ok(s) = bincode::serialize(deltavec) {
                s
            } else {
                return Err("Could not serialize deltas!".to_string());
            };
            header = PktHeader {tag: PktType::EnemyDelta, payload_len: paybuf.len()};
        }
    }
    let io_header = IoSlice::new(unsafe {
        std::slice::from_raw_parts((&header as *const PktHeader)
            as *const u8, std::mem::size_of::<PktHeader>())
    });
    let io_payload = IoSlice::new(paybuf.as_slice());
    match stream.write_vectored(&[io_header, io_payload]) {
        Ok(sz) => {
            Ok(sz)
        }
        Err(ref e) => {
            match e.kind() {
                ErrorKind::WouldBlock |
                ErrorKind::Interrupted => {
                    Err("Could not write to stream!".to_string())
                }
                _ => {
                    Err("Fatal".to_string())
                }
            }
        }
    }
}
