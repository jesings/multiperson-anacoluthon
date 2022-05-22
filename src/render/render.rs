use crate::gamestate;
use crate::entity::entity::Entity;

use sdl2::pixels::Color;
use sdl2::rect::*;

const WINDOWWIDTH: u32 = 320;
const WINDOWHEIGHT: u32 = 240;

const SIDEWIDTH: u32 = 80;
pub const TILEWIDTH: u32 = 16;
pub const ITILEWIDTH: i32 = TILEWIDTH as i32;

impl gamestate::ClientGamestate<'_> {
    pub fn render(&self) {
        let pid = self.pid;

        let gd = &self.gamedata;

        let pos = gd.players[pid].lock().expect("Could not lock player to get its position.").pos().clone();

        let mut canv = self.sdl.canv.lock().expect("Could not lock canvas for rendering!");
        canv.set_draw_color(Color::RGB(0, 0, 0));
        canv.clear();
        let textures = &self.sdl.texture_table;

        //get dimensions of the canvas
        let canvsize: (u32, u32) = canv.output_size().expect("Could not get canvas size.");

        let upscale: u32 = std::cmp::min(canvsize.0 / WINDOWWIDTH, canvsize.1 / WINDOWHEIGHT);
        let scaled_w: i32 = (upscale * WINDOWWIDTH) as i32;
        let scaled_h: i32 = (upscale * WINDOWHEIGHT) as i32;

        //get corners of grid from that
        let left_x: i32 = (canvsize.0 as i32 - scaled_w) / 2; //initialize to corner position
        let left_tile_x: i32 = left_x + (SIDEWIDTH * upscale) as i32;
        let top_y: i32 = (canvsize.1 as i32 - scaled_h) / 2; //initialize to corner position

        let mut rendrect = Rect::new(left_tile_x, top_y, TILEWIDTH * upscale, TILEWIDTH * upscale);

        let player_tile_start_x: i32 = left_tile_x + (WINDOWWIDTH - SIDEWIDTH) as i32 / 2 / ITILEWIDTH * ITILEWIDTH * upscale as i32;
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

                    textures.draw_tile(&mut canv, rendrect, tile.texture);
                } else {
                    canv.set_draw_color(Color::RGB(0, 0, 0));
                    if let Err(_) = canv.fill_rect(rendrect) {
                        eprintln!("Could not render tile at ({}, {}) from grid", col_index, row_index);
                    }

                }

                rendrect.offset(0, ITILEWIDTH * upscale as i32);
            }
            rendrect.offset(ITILEWIDTH * upscale as i32, 0);
        }

        let curplayer_rect = Rect::new(player_tile_start_x, player_tile_start_y, TILEWIDTH * upscale, TILEWIDTH * upscale);

        //now draw the players
        for wrappedplayer in &gd.players {
            let player = wrappedplayer.lock().expect("Could not lock player to get its position.");
            let rightrect;

            if player.pid == pid {
                //for the current player, just draw it in the center of the screen
                rightrect = curplayer_rect.clone();
            } else {
                //for the other players, find their tile offset from the current player and render them from that

                let otherpos = player.pos();
                let xdelta = (pos.0 - otherpos.0) as i32;
                let ydelta = (pos.1 - otherpos.1) as i32;
                rightrect = Rect::new(
                    player_tile_start_x - xdelta * ITILEWIDTH * upscale as i32,
                    player_tile_start_y - ydelta * ITILEWIDTH * upscale as i32,
                    TILEWIDTH,
                    TILEWIDTH,
                );
            }

            textures.draw_player(&mut canv, rightrect, player.pid as i32);
        }

        //draw ui
        canv.set_draw_color(Color::RGB(66, 66, 66));
        if let Err(_) = canv.fill_rect(Rect::new(left_x, top_y, SIDEWIDTH * upscale, WINDOWHEIGHT * upscale)) {
            eprintln!("UI draw failure");
        };
        textures.draw_portrait(&mut canv, Rect::new(left_x + 10 * upscale as i32, top_y + 2 * upscale as i32, 60* upscale, 60 * upscale));
        
        canv.present();
    }
}
