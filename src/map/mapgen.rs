use rand::prelude::*;
use std::sync::*;
use super::grid::*;


impl Grid {
    pub fn gen_blank_grid(height: usize, width: usize) -> Self {

        let dummytile = Arc::new(Tile {
            texture: 0,
            passable: true,
            transparent: true
        });

        let gridtiles = (0..height*width).map(|_| dummytile.clone()).collect();

        Self {
            cols: height,
            rows: width,
            tiles: gridtiles
        }
    }
    pub fn gen_cell_auto(height: usize, width: usize, seed: [u8; 32]) -> Self {
        let tileset = (0..=255u8).map(|x| Arc::new(Tile {
            texture: x,
            passable: true,
            transparent: true
        })).collect::<Vec<Arc<Tile>>>();

        let mut ayn = rand_xoshiro::Xoshiro256Plus::from_seed(seed);

        //let deathlimit = 3;
        //let birthlimit = 3;

        let mut gridtiles = vec!();
        for _ in 0..(height*width) {
            gridtiles.push(tileset[ayn.next_u64() as usize % 256].clone());
        }

        Self {
            cols: height,
            rows: width,
            tiles: gridtiles
        }
    }
}
