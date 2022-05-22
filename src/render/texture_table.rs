use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::rect::Rect;

use std::path::Path;
use std::sync::Arc;

use super::render::{TILEWIDTH, ITILEWIDTH};

pub struct TextureTable<'a> {
    
    tiles_texture: Texture<'a>,
    player_texture: Texture<'a>,
    portrait_texture: Texture<'a>,
}

impl<'a> TextureTable<'a> {
    pub fn init(texture_creator: Arc<TextureCreator<sdl2::video::WindowContext>>) -> Self {
        let tiles_texture = texture_creator.clone().load_texture(Path::new("textures/tiles_placeholder.bmp")).unwrap();
        let player_texture = texture_creator.clone().load_texture(Path::new("textures/player_placeholder.bmp")).unwrap();
        let portrait_texture = texture_creator.clone().load_texture(Path::new("textures/portrait_placeholder.bmp")).unwrap();
        TextureTable {
            tiles_texture,
            player_texture,
            portrait_texture,
        }
    }

    pub fn draw_player(&self, c: &mut Canvas<sdl2::video::Window>, dst: Rect, index: i32) {
        c.copy(&self.player_texture, Rect::new(index * ITILEWIDTH, 0, TILEWIDTH, TILEWIDTH), dst);
    }

    pub fn draw_portrait(&self, c: &mut Canvas<sdl2::video::Window>, dst: Rect) {
        c.copy(&self.portrait_texture, None, dst);
    }

    pub fn draw_tile(&self, c: &mut Canvas<sdl2::video::Window>, dst: Rect, index: i32) {
        c.copy(&self.tiles_texture, Rect::new(index * ITILEWIDTH, 0, TILEWIDTH, TILEWIDTH), dst);
    }
}
