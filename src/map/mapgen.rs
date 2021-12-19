use std::sync::*;
use super::grid::*;


impl Grid {
    pub fn gen_blank_grid(height: usize, width: usize) -> Self {
        let mut gridtiles = vec!();

        let dummytile = Arc::new(Tile {
            texture: 0,
            passable: true,
            transparent: true
        });

        for _ in 0..(height*width) {
            gridtiles.push(dummytile.clone());
        }

        Self {
            cols: height,
            rows: width,
            tiles: gridtiles
        }
    }
}
