use crate::gamestate;
use sdl2::pixels::Color;
use sdl2::rect::*;

const WINDOWWIDTH: u32 = 320;
const WINDOWHEIGHT: u32 = 240;

const TILEWIDTH: u32 = 16;
const ITILEWIDTH: i32 = TILEWIDTH as i32;

const PLAYERWIDTH: u32 = TILEWIDTH / 2;
const IPLAYERWIDTH: i32 = ITILEWIDTH / 2;

impl gamestate::ClientGamestate {
    pub fn render(&self) {
        let pid = self.pid;

        let gd = &self.gamedata;

        let pos = gd.players[pid].lock().expect("Could not lock player to get its position.").pos.clone();

        let mut canv = self.sdl.canv.lock().expect("Could not lock canvas for rendering!");
        canv.clear();

        //get dimensions of the canvas
        let canvsize: (u32, u32) = canv.output_size().expect("Could not get canvas size.");

        let upscale: u32 = std::cmp::min(canvsize.0 / WINDOWWIDTH, canvsize.1 / WINDOWHEIGHT);
        let scaled_w: i32 = (upscale * WINDOWWIDTH) as i32;
        let scaled_h: i32 = (upscale * WINDOWHEIGHT) as i32;

        //get corners of grid from that
        let left_x: i32 = (canvsize.0 as i32 - scaled_w) / 2; //initialize to corner position
        let top_y: i32 = (canvsize.1 as i32 - scaled_h) / 2; //initialize to corner position

        let mut rendrect = Rect::new(left_x, top_y, TILEWIDTH * upscale, TILEWIDTH * upscale);

        let player_tile_start_x: i32 = left_x + WINDOWWIDTH as i32 / 2 / ITILEWIDTH * ITILEWIDTH * upscale as i32;
        let player_tile_start_y: i32 = top_y + WINDOWHEIGHT as i32 / 2 / ITILEWIDTH * ITILEWIDTH * upscale as i32;

        //render tiles between corners
        while rendrect.x() < left_x + scaled_w {
            rendrect.set_y(top_y);
            let col_index = (rendrect.x() - player_tile_start_x) / (ITILEWIDTH * upscale as i32) + pos.0 as i32;
            while rendrect.y() < top_y + scaled_h {
                //eventually use copy or copy_ex for textures, get from the grid coord

                //this better be strength reduced by the compiler or rust is kinda lamer
                let row_index = (rendrect.y() - player_tile_start_y) / (ITILEWIDTH * upscale as i32) + pos.1 as i32;
                //now we have the position of the tile from its location relative to the player

                if col_index >= 0 && row_index >= 0 && col_index < gd.grid.cols as i32 && row_index < gd.grid.rows as i32 {
                    let grid_index = row_index as usize * gd.grid.cols + col_index as usize;
                    let tile = &gd.grid.tiles[grid_index];
                    let rshade = tile.texture;

                    canv.set_draw_color(Color::RGB(rshade, rshade, rshade));
                } else {
                    canv.set_draw_color(Color::RGB(0, 0, 0));
                }

                if let Err(_) = canv.fill_rect(rendrect) {
                    eprintln!("Could not render tile at ({}, {}) from grid", col_index, row_index);
                }

                rendrect.offset(0, ITILEWIDTH * upscale as i32);
            }
            rendrect.offset(ITILEWIDTH * upscale as i32, 0);
        }

        let p_margin = IPLAYERWIDTH * upscale as i32 / 2;
        let curplayer_rect = Rect::new(player_tile_start_x + p_margin, player_tile_start_y + p_margin, PLAYERWIDTH * upscale, PLAYERWIDTH * upscale);

        //now draw the players
        for wrappedplayer in &gd.players {
            let player = wrappedplayer.lock().expect("Could not lock player to get its position.");
            let rightrect;

            if player.pid == pid {
                //for the current player, just draw it in the center of the screen
                canv.set_draw_color(Color::RGB(0, 255, 0));
                rightrect = curplayer_rect.clone();
            } else {
                //for the other players, find their tile offset from the current player and render them from that
                canv.set_draw_color(Color::RGB(255, 0, 0));
                let otherpos = player.pos;
                let xdelta = (pos.0 - otherpos.0) as i32;
                let ydelta = (pos.1 - otherpos.1) as i32;
                rightrect = Rect::new(
                    player_tile_start_x - xdelta * ITILEWIDTH * upscale as i32 + p_margin,
                    player_tile_start_y - ydelta * ITILEWIDTH * upscale as i32 + p_margin,
                    PLAYERWIDTH,
                    PLAYERWIDTH,
                );
            }

            if let Err(_) = canv.fill_rect(rightrect) {
                eprintln!("Could not render player {}", player.pid);
            }
        }

        canv.present();
    }
}
