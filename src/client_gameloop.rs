use crate::gamestate;
use crate::client_netloop;
use crate::net::*;
use crate::map::grid::*;

use sdl2::*;

use std::sync::*;
use std::time::{Duration, Instant};
use std::thread;

const FRAMERATE: u32 = 60;

const IPADDR: &str = "127.0.0.1";
const PORT: u16 = 9495;

fn init_game() -> gamestate::ClientGamestate {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("AMS2", 640, 480).position_centered().build().unwrap();

    let canvas = window.into_canvas().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();
    let ipstr = format!("{}:{}", IPADDR, PORT);
    let mut upstream = clinet::initialize_client(ipstr);
    let mut gdt = if let pkt::PktPayload::Gamedata(fgd) = pkt::recv_pkt(&mut upstream).expect("Did not recieve gamedata during initialization!") {
        fgd
    } else {
        panic!("Incorrect packet type recieved during initialization");
    };
    upstream.set_nonblocking(true).unwrap();
    //generate grid from gdt seed
    let pid = gdt.2;
    let gamedata =  Arc::new(gamestate::Gamedata {
        players: gdt.0.drain(..).map(|x| Arc::new(Mutex::new(x))).collect(),
        grid: Grid::gen_blank_grid(480, 640),
    });

    let gdc = gamedata.clone();
    let runningstatebool = Arc::new(atomic::AtomicBool::new(true));
    let rsbc = runningstatebool.clone();
    let (sender, recver) = mpsc::channel();
    let handle = thread::spawn(move || {
        client_netloop::netloop(upstream, gdc, pid, rsbc, recver)
    });
    gamestate::ClientGamestate {
        handle,
        runningstate: runningstatebool,
        sdl: gamestate::Sdlstate {
            ctx: sdl_context,
            vid: video_subsystem,
            pump: Mutex::new(event_pump),
            canv: Mutex::new(canvas),
        },
        pid,
        gamedata,
        sender
    }
}

pub fn gameloop() -> Result<(), String> {
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
                    gs.runningstate.store(false, atomic::Ordering::Relaxed);
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
            (callback)(gs.gamedata.clone(), &gs.sender);
        }
        // println!("{:?}", gs.gamedata.players[gs.pid].lock().unwrap());
        // The rest of the game loop goes here...

        gs.render();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMERATE));
    }

    gs.handle.join().unwrap().unwrap();

    return Ok(());
}
