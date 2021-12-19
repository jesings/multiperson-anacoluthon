use super::grid::*;

impl Grid {
    pub fn gen_blank_grid(height: usize, width: usize) -> Self {
        let mut gridtiles = vec!();
        for _ in 0..(height*width) {
            gridtiles.push(Tile {
                texture: 0,
                passable: true,
                transparent: true
            });
        }

        Self {
            cols: height,
            rows: width,
            tiles: gridtiles
        }
    }
}
