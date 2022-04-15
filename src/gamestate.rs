use serde::{Serialize, Deserialize};
use sdl2::*;
use std::sync::{*, mpsc};
use std::thread::JoinHandle;

use super::map::grid::Grid;
use super::player::player::Player;
use super::net::pkt::PktPayload;


//GameState will always be wrapped in an arc, so its immutable members can be accessed without a lock or arc at all?
//Other, mutable members can either be wrapped in a struct or not
pub struct ClientGamestate {
    pub handle: JoinHandle<Result<(), String>>,
    pub runningstate: Arc<atomic::AtomicBool>,
    pub sdl: Sdlstate,
    pub pid: usize,
    pub gamedata: Arc<Gamedata>,
    pub sender: mpsc::Sender<PktPayload>,
}

// todo: jonathicc decide what the mutex guards guard
#[derive(Debug)]
pub struct Gamedata {
    pub players: Vec<Arc<Mutex<Player>>>,
    pub grid: Grid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitializationData{
    pub players: Vec<Player>,
    pub pid: Option<usize>,
    pub seed: [u8; 32],
}

//gamedata struct, shared between client and server?
//nothing needs it as mut
pub struct Sdlstate {
    pub ctx: Sdl,
    pub vid: VideoSubsystem,
    pub pump: Mutex<EventPump>,
    pub canv: Mutex<render::Canvas<video::Window>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DeltaEvent {
    pub pid: usize,
    pub poschange: (isize, isize),
}
