use rand::prelude::*; 
use std::sync::*;
use super::grid::*;

macro_rules! eightdir {
    //first arg is function name, second is any arguments before x and y
    ($func: ident, $xname: ident, $yname: ident, $joiner: tt, $($otherarg: expr)*) =>{
        $func($($otherarg),*, $xname - 1, $yname - 1) $joiner
        $func($($otherarg),*, $xname, $yname - 1) $joiner
        $func($($otherarg),*, $xname - 1, $yname) $joiner
        $func($($otherarg),*, $xname + 1, $yname - 1) $joiner
        $func($($otherarg),*, $xname - 1, $yname + 1) $joiner
        $func($($otherarg),*, $xname + 1, $yname) $joiner
        $func($($otherarg),*, $xname, $yname + 1) $joiner
        $func($($otherarg),*, $xname + 1, $yname + 1)
    };
}


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

        let deathlimit = 4;
        let birthlimit = 3;
        let liveness_threshold = 150u8;

        loop {
            let inbounds = |x, y| x < width as isize && y < height as isize && x >= 0 && y >= 0;
            let neighborstate = |v: &Vec<u8>, x, y| inbounds(x, y) && (v[(y as usize)*width+(x as usize)] > liveness_threshold);
            let neighborstate_int = |v: &Vec<u8>, x, y| neighborstate(v, x, y) as i32;

            let liveneighbor_count = |v: &Vec<u8>, x, y| eightdir!(neighborstate_int, x, y, +, v);

            let cellularstep = |before: &mut Vec<u8>, after: &mut Vec<u8>| {
                for y in 0..(height as isize) {
                    for x in 0..(width as isize) {
                        let num_alive = liveneighbor_count(before, x, y);
                        let index = y as usize * width + x as usize;
                        if before[index] > liveness_threshold {
                            after[index] = if num_alive < deathlimit {255u8} else {0u8};
                        } else {
                            after[index] = if num_alive < birthlimit {0u8} else {255u8};
                        }
                    }
                }
            };

            let mut startvec = (0..height*width).map(|_| (ayn.next_u32() % 256) as u8).collect();
            let mut scratchvec = vec![0u8; height*width];

            for _ in 1..5 {
                cellularstep(&mut startvec, &mut scratchvec);
                cellularstep(&mut scratchvec, &mut startvec);
            }
            let gridtiles = startvec.drain(..).map(|i| tileset[i as usize].clone()).collect();

            return Self {
                cols: height,
                rows: width,
                tiles: gridtiles
            };
        }

    }
}
