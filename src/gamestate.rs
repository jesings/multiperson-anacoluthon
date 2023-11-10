use serde::{Serialize, Deserialize};
use sdl2::*;
use std::sync::{*, mpsc};
use std::thread::JoinHandle;
use std::collections::HashMap;
use std::collections::BTreeMap;

use super::map::grid::Grid;
use super::player::player::Player;
use super::enemy::enemy::Enemy;
use super::bozo_ent::bozo_ent::BozoEnt;
use super::entity::entity::{Etype};
use super::net::pkt::{PktType, PktPayload};
use super::render::texture_table::TextureTable;


//GameState will always be wrapped in an arc, so its immutable members can be accessed without a lock or arc at all?
//Other, mutable members can either be wrapped in a struct or not
pub struct ClientGamestate<'a> {
    pub handle: JoinHandle<Result<(), String>>,
    pub runningstate: Arc<atomic::AtomicBool>,
    pub sdl: Sdlstate<'a>,
    pub pid: usize,
    pub gamedata: Arc<Gamedata>,
    pub sender: mpsc::Sender<PktPayload>,
}

// todo: jonathicc decide what the mutex guards guard
#[derive(Debug)]
pub struct Gamedata {
    pub players: Vec<Arc<Mutex<Player>>>,
    pub enemies: Vec<Arc<Mutex<Enemy>>>,
    pub bozoents: BTreeMap<usize, Arc<Mutex<BozoEnt>>>,
    pub grid: Grid,
    pub occupation: Arc<RwLock<HashMap<(isize, isize), Vec<(Etype, usize)>>>>,
    pub pktbuf: Arc<Mutex<BTreeMap<PktType, PktPayload>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitializationData{
    pub players: Vec<Player>,
    pub enemies: Vec<Enemy>,
    pub pid: Option<usize>,
    pub seed: [u8; 32],
}

//gamedata struct, shared between client and server?
//nothing needs it as mut
pub struct Sdlstate<'a> {
    pub ctx: Sdl,
    pub vid: VideoSubsystem,
    pub pump: Mutex<EventPump>,
    pub canv: Mutex<render::Canvas<video::Window>>,
    pub texture_table: TextureTable<'a>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerDeltaEvent {
    pub pid: usize,
    pub newpos: (isize, isize),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct EnemyDeltaEvent {
    pub eid: usize,
    pub newpos: (isize, isize),
}
