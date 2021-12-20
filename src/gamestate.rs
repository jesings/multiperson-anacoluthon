use sdl2::*;
use std::sync::*;
use super::net::clinet::ClientNetstate;
use super::map::grid::Grid;
use super::player::player::Player;


//GameState will always be wrapped in an arc, so its immutable members can be accessed without a lock or arc at all?
//Other, mutable members can either be wrapped in a struct or not
pub struct ClientGamestate {
    //pub net: ClientNetstate,
    pub sdl: Sdlstate,
    pub pid: usize,
    pub gamedata: Arc<Gamedata>,
}

// todo: jonathicc decide what the mutex guards guard
pub struct Gamedata {
    pub players: Vec<Player>,
    pub grid: Grid,
}

//gamedata struct, shared between client and server?
//nothing needs it as mod
pub struct Sdlstate {
    pub ctx: Sdl,
    pub vid: VideoSubsystem,
    pub pump: Mutex<EventPump>,
    pub canv: Mutex<render::Canvas<video::Window>>,
}
