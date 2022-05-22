use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;

use std::path::Path;

use super::render::{TILEWIDTH, ITILEWIDTH};

pub struct TextureTable<'a> {
    tiles_texture: Texture<'a>,
    player_texture: Texture<'a>,
    portrait_texture: Texture<'a>,
}

impl<'a> TextureTable<'a> {
    pub fn init(texture_creator: &'a TextureCreator<sdl2::video::WindowContext>) -> Self {
        let tiles_texture = texture_creator.load_texture(Path::new("textures/tiles_placeholder.png")).unwrap();
        let player_texture = texture_creator.load_texture(Path::new("textures/player_placeholder.png")).unwrap();
        let portrait_texture = texture_creator.load_texture(Path::new("textures/portrait_placeholder.png")).unwrap();
        TextureTable {
            tiles_texture,
            player_texture,
            portrait_texture,
        }
    }

    pub fn draw_player(&self, c: &mut Canvas<sdl2::video::Window>, dst: Rect, index: i32) {
        if let Err(_) = c.copy(&self.player_texture, Rect::new(index % 4 * ITILEWIDTH, 0, TILEWIDTH, TILEWIDTH), dst) {
            eprintln!("Could not render tile");
        }
    }

    pub fn draw_enemy(&self, c: &mut Canvas<sdl2::video::Window>, dst: Rect) {
        if let Err(_) = c.copy(&self.player_texture, Rect::new(0, ITILEWIDTH, TILEWIDTH, TILEWIDTH), dst) {
            eprintln!("Could not render tile");
        }
    }
    pub fn draw_portrait(&self, c: &mut Canvas<sdl2::video::Window>, dst: Rect) {
        if let Err(_) = c.copy(&self.portrait_texture, None, dst) {
            eprintln!("Could not render tile");
        }
    }

    pub fn draw_tile(&self, c: &mut Canvas<sdl2::video::Window>, dst: Rect, index: i32) {
        if let Err(_) = c.copy(&self.tiles_texture, Rect::new(ITILEWIDTH * index, 0, TILEWIDTH, TILEWIDTH), dst) {
            eprintln!("Could not render tile");
        }
    }
}
