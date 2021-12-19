use std::sync::*;
use super::net::clinet::ClientNetstate;

//GameState will always be wrapped in an arc, so its immutable members can be accessed without a lock or arc at all?
//Other, mutable members can either be wrapped in a struct or not
struct ClientGameState {
    pub net: ClientNetstate,
    //pub gamedata
    //sdl_garbage
}

//gamedata struct, shared between client and server?
