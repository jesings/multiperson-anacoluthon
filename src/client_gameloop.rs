use crate::gamestate;
use crate::net;
use crate::player::player::*;
use crate::map::grid::*;
use crate::render::render::*;
use crate::control::control::*;

use sdl2::*;

use std::sync::*;
use std::time::{Duration, Instant};

fn init_game() -> gamestate::ClientGamestate {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("AMS2", 640, 480).position_centered().build().unwrap();

    let canvas = window.into_canvas().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();
    gamestate::ClientGamestate {
        sdl: gamestate::Sdlstate {
            ctx: sdl_context,
            vid: video_subsystem,
            pump: Mutex::new(event_pump),
            canv: Mutex::new(canvas),
        },
        pid: 0,
        gamedata: Arc::new(gamestate::Gamedata {
            players: vec![Arc::new(Mutex::new(Player::test_player(0)))],
            grid: Grid::gen_blank_grid(480, 640),
        }),
    }
}

pub fn gameloop() {
    let gs = init_game();
    let mut controller = Controller::new();
    let start = Instant::now();
    let mut i = 0;
    'running: loop {
        let now = Instant::now();
        let gametime = now.checked_duration_since(start).unwrap();
        i = (i + 1) % 255;
        
        if !controller.control(&gs.sdl.pump, gametime, gs.gamedata.clone(), gs.pid) {
            break 'running;
        }
        
        // println!("{:?}", gs.gamedata.players[gs.pid].lock().unwrap());
        // The rest of the game loop goes here...

        gs.render();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
