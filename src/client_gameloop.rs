use crate::gamestate;
use crate::net;
use crate::player::player::*;
use crate::map::grid::*;
use crate::render::render::*;

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

    let start = Instant::now();
    let mut i = 0;
    'running: loop {
        let now = start.elapsed();
        i = (i + 1) % 255;
        let mut callstack = vec![];
        for event in gs.sdl.pump.lock().unwrap().poll_iter() {
            match event {
                event::Event::Quit {..} |
                event::Event::KeyDown { keycode: Some(keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
                event::Event::KeyDown {keycode: Some(keyboard::Keycode::W), repeat: false, .. } => {
                    callstack.push(gs.gamedata.players[gs.pid].lock().unwrap().class.mov(gs.pid, (0, -1), now));
                },
                event::Event::KeyDown {keycode: Some(keyboard::Keycode::A), repeat: false, .. } => {
                    callstack.push(gs.gamedata.players[gs.pid].lock().unwrap().class.mov(gs.pid, (-1, 0), now));
                },
                event::Event::KeyDown {keycode: Some(keyboard::Keycode::S), repeat: false, .. } => {
                    callstack.push(gs.gamedata.players[gs.pid].lock().unwrap().class.mov(gs.pid, (0, 1), now));
                },
                event::Event::KeyDown {keycode: Some(keyboard::Keycode::D), repeat: false, .. } => {
                    callstack.push(gs.gamedata.players[gs.pid].lock().unwrap().class.mov(gs.pid, (1, 0), now));
                },
                _ => {}
            };
        }
        for callback in callstack {
            (callback)(gs.gamedata.clone());
        }
        // println!("{:?}", gs.gamedata.players[gs.pid].lock().unwrap());
        // The rest of the game loop goes here...

        gs.render();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
