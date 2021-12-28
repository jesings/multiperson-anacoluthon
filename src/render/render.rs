use crate::map::grid;
use crate::gamestate;
use sdl2::rect::*;
use sdl2::pixels::Color;
use rand::prelude::*;
use rand::rngs::*;

const TILEWIDTH: u32 = 64;
const ITILEWIDTH: i32 = TILEWIDTH as i32;

const PLAYERWIDTH: u32 = 30;
const IPLAYERWIDTH: i32 = 30;

impl gamestate::ClientGamestate {
    pub fn render(&self) {
        let pid = self.pid;

        let gd = &self.gamedata;

        let pos = gd.players[pid].lock().expect("Could not lock player to get its position.").pos.clone();

        let mut canv = self.sdl.canv.lock().expect("Could not lock canvas for rendering!");
        //canv.clear(); probably unneeded

        //get dimensions of the canvas
        let canvsize: (u32, u32) = canv.output_size().expect("Could not get canvas size.");
        let icanvsize = (canvsize.0 as i32, canvsize.1 as i32);

        //get corners of grid from that
        let left_x = -((icanvsize.0 % ITILEWIDTH + ITILEWIDTH) / 2); //initialize to corner position
        let top_y = -((icanvsize.1 % ITILEWIDTH + ITILEWIDTH) / 2); //initialize to corner position
        let mut rendrect = Rect::new(left_x, top_y, TILEWIDTH, TILEWIDTH);

        let player_tile_start_x = (icanvsize.0 / 2 - left_x - 1) / ITILEWIDTH * ITILEWIDTH + left_x; //rounded up
        let player_tile_start_y = (icanvsize.1 / 2 - top_y - 1) / ITILEWIDTH * ITILEWIDTH + top_y; //rounded up

        //render tiles between corners
        while rendrect.x() < icanvsize.0 {
            rendrect.set_y(top_y);
            let xpos = (rendrect.x() - player_tile_start_x) / ITILEWIDTH + pos.0 as i32;
            while rendrect.y() < icanvsize.1 {
                //eventually use copy or copy_ex for textures, get from the grid coord
                
                let ypos = (rendrect.y() - player_tile_start_y) / ITILEWIDTH + pos.1 as i32;
                //now we have the position of the tile from its location relative to the player

                if xpos >= 0 && ypos >= 0 {
                     let mut seed = [0u8; 32];
                     for (index, b) in xpos.to_ne_bytes().iter().enumerate() {
                         seed[index] = *b;
                     }
                     for (index, b) in ypos.to_ne_bytes().iter().enumerate() {
                         seed[index + 4] = *b;
                     }
                     let mut sillyrng = StdRng::from_seed(seed);

                     let rshade = (sillyrng.next_u32() % 256) as u8;

                     canv.set_draw_color(Color::RGB(rshade, rshade, rshade));
                } else {
                     canv.set_draw_color(Color::RGB(0, 0, 0));
                }
                
                if let Err(_) = canv.fill_rect(rendrect) {
                    eprintln!("Could not render tile at ({}, {}) from grid", xpos, ypos);
                }

                rendrect.offset(0, ITILEWIDTH);
            }
            rendrect.offset(ITILEWIDTH, 0);
        }

        let curplayer_rect = Rect::new((icanvsize.0 - IPLAYERWIDTH) / 2, (icanvsize.1 - IPLAYERWIDTH) / 2, PLAYERWIDTH, PLAYERWIDTH);
        for wrappedplayer in &gd.players {
            let player = wrappedplayer.lock().expect("Could not lock player to get its position.");
            let rightrect;
            if player.pid == pid {
                canv.set_draw_color(Color::RGB(0, 255, 0));
                rightrect = curplayer_rect.clone();
            } else {
                canv.set_draw_color(Color::RGB(255, 0, 0));
                let otherpos = player.pos;
                let xdelta = (otherpos.0 - pos.0) as i32;
                let ydelta = (otherpos.1 - pos.1) as i32;
                rightrect = Rect::new((icanvsize.0 - IPLAYERWIDTH) / 2 - xdelta * ITILEWIDTH, (icanvsize.1 - IPLAYERWIDTH) / 2 - ydelta * ITILEWIDTH, PLAYERWIDTH, PLAYERWIDTH);
            }
            if let Err(_) = canv.fill_rect(rightrect) {
                eprintln!("Could not render player {}", player.pid);
            }
        }
        
        //present canvas
        canv.present();
    }
}
