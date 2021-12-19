use crate::gamestate;
use crate::net;

use sdl2::*;

use std::sync::*;

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
    }
}

pub fn gameloop() {
    let gs = init_game();
    let mut canv = gs.sdl.canv.lock().expect("Could not unlock canvas!");
    canv.set_draw_color(pixels::Color::RGB(255, 255, 255));
    canv.clear();
    canv.present();
    drop(canv);

    let mut i = 0;
    'running: loop {
        let mut canvas = gs.sdl.canv.lock().expect("could not unlock canvas");
        i = (i + 1) % 255;
        canvas.set_draw_color(pixels::Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in gs.sdl.pump.lock().unwrap().poll_iter() {
            match event {
                event::Event::Quit {..} |
                event::Event::KeyDown { keycode: Some(keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
      }

}
