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

        let mut ayn = rand_xoshiro::Xoshiro256PlusPlus::from_seed(seed);

        let deathlimit = 3;
        let birthlimit = 4;
        let liveness_threshold = 162u8;

        loop {
            let inbounds = |x, y| x < width as isize && y < height as isize && x >= 0 && y >= 0;
            let neighborstate = |v: &Vec<u8>, x, y| inbounds(x, y) && (v[(y as usize)*width+(x as usize)] > liveness_threshold);
            let neighborstate_int = |v: &Vec<u8>, x, y| neighborstate(v, x, y) as i32;

            let liveneighbor_count = |v: &Vec<u8>, x, y| eightdir!(neighborstate_int, x, y, +, v);

            let cellularstep = |before: &Vec<u8>, after: &mut Vec<u8>| {
                for y in 0..(height as isize) {
                    for x in 0..(width as isize) {
                        let num_alive = liveneighbor_count(before, x, y);
                        let index = y as usize * width + x as usize;
                        if before[index] > liveness_threshold {
                            after[index] = if num_alive < deathlimit {0u8} else {255u8};
                        } else {
                            after[index] = if num_alive < birthlimit {0u8} else {255u8};
                        }
                    }
                }
            };

            let mut startvec: Vec<u8> = vec![0; height*width];
            ayn.fill_bytes(startvec.as_mut_slice());
            let mut scratchvec = vec![0u8; height*width];

            for _ in 1..3 {
                cellularstep(&startvec, &mut scratchvec);
                cellularstep(&scratchvec, &mut startvec);
            }

            for _ in 0..16 {
                let xd = ayn.next_u64() as usize % width;
                let yd = ayn.next_u64() as usize % height;
                let index = yd * width + xd;
                if startvec[index] < liveness_threshold {
                    continue;
                }
                scratchvec = vec![0u8; height*width];

                let mut filled_count = 0;

                let mut fillstack: Vec<(isize, isize)> = vec!();
                fillstack.push((xd as isize , yd as isize));
                let condpush = |x, y, coordstack: &mut Vec<(isize, isize)>| {
                    if neighborstate(&startvec, x, y) {
                        coordstack.push((x, y));
                    }
                };

                let mut floodfill = |x, y, coordstack: &mut Vec<(isize, isize)>| {
                    let floodindex = y as usize * width + x as usize;
                    if scratchvec[floodindex] == 0 {
                        filled_count += 1;
                        scratchvec[floodindex] = 255;
                        condpush(x - 1, y, coordstack);
                        condpush(x + 1, y, coordstack);
                        condpush(x, y + 1, coordstack);
                        condpush(x, y - 1, coordstack);
                    }
                };

                while let Some(tuple) = fillstack.pop() {
                    floodfill(tuple.0, tuple.1, &mut fillstack);
                }
                if filled_count > width * height * 2 / 5 {
                    let gridtiles = scratchvec.drain(..).map(|i| tileset[i as usize].clone()).collect();

                    return Self {
                        cols: width,
                        rows: height,
                        tiles: gridtiles
                    };
                }
            }
            eprintln!("Grid generation failed! Retrying to generate grid!");
        }

    }
}
