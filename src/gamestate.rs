use serde::{Serialize, Deserialize};
use sdl2::*;
use std::sync::{*, mpsc::*};
use std::thread::JoinHandle;
use std::net::*;

use super::net::pkt::PktPayload;
use super::map::grid::Grid;
use super::player::player::Player;


//GameState will always be wrapped in an arc, so its immutable members can be accessed without a lock or arc at all?
//Other, mutable members can either be wrapped in a struct or not
pub struct ClientGamestate {
    pub stream: TcpStream,
    pub sdl: Sdlstate,
    pub pid: usize,
    pub gamedata: Arc<Gamedata>,
}

//multithread gamedata soon
pub struct ServerGamestate {
    pub workers: Vec<JoinHandle<Result<(), String>>>,
    pub transmitter: bus::Bus<Arc<PktPayload>>,
    pub reciever: Receiver<PktPayload>,
    pub gamedata: Arc<Gamedata>,
}

// todo: jonathicc decide what the mutex guards guard
pub struct Gamedata {
    pub players: Vec<Arc<Mutex<Player>>>,
    pub grid: Grid,
}

#[derive(Serialize, Deserialize)]
pub struct GDTuple(pub Vec<Player>, pub i128); //the i128 is the seed for mapgen

//gamedata struct, shared between client and server?
//nothing needs it as mod
pub struct Sdlstate {
    pub ctx: Sdl,
    pub vid: VideoSubsystem,
    pub pump: Mutex<EventPump>,
    pub canv: Mutex<render::Canvas<video::Window>>,
}

#[derive(Serialize, Deserialize)]
pub struct DeltaEvent {
    pub pid: usize,
    pub poschange: (isize, isize),
}
