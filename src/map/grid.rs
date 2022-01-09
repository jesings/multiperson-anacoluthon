use std::sync::*;

#[derive(Debug)]
pub struct Tile {
    pub texture: u8, //Not sure texture should be a u8, we can make it an SDL object later
    pub passable: bool,
    pub transparent: bool,
}

#[derive(Debug)]
pub struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub tiles: Vec<Arc<Tile>>,
}
