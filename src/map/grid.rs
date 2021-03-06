use std::sync::*;

pub const MAPDIM: (usize, usize) = (200, 200);

#[derive(Debug)]
pub struct Tile {
    pub texture: i32,
    pub passable: bool,
    pub transparent: bool,
}

#[derive(Debug)]
pub struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub tiles: Vec<Arc<Tile>>,
}

impl Grid {
    pub fn passable(&self, tilepos: (isize, isize)) -> bool {
        if tilepos.0 < 0 || tilepos.1 < 0 {
            return false;
        }
        let p0 = tilepos.0 as usize;
        let p1 = tilepos.1 as usize;
        if p0 >= self.cols || p1 >= self.rows {
            return false
        }
        let index = p1 * self.cols + p0;
        return self.tiles[index].passable;
    }
}
