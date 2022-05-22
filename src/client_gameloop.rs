use crate::gamestate;
use crate::client_netloop;
use crate::net::*;
use crate::map::grid::*;
use crate::control::control::*;

use std::sync::*;
use std::time::{Duration, Instant};
use std::thread;

const FRAMERATE: u32 = 60;

const IPADDR: &str = "127.0.0.1";
const PORT: u16 = 9495;

pub fn gameloop() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("AMS2", 640, 480).resizable().position_centered().build().unwrap();
    //yo we could set window title and icon using set_title and set_icon

    let canvas = window.into_canvas().accelerated().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();
    let ipstr = format!("{}:{}", IPADDR, PORT);
    let mut upstream = clinet::initialize_client(ipstr);
    let mut initdata = if let pkt::PktPayload::Initial(fgd) = pkt::recv_pkt(&mut upstream).expect("Did not recieve gamedata during initialization!") {
        fgd
    } else {
        panic!("Incorrect packet type recieved during initialization");
    };
    upstream.set_nonblocking(true).unwrap();
    let pid = initdata.pid.unwrap();
    let gamedata =  Arc::new(gamestate::Gamedata {
        players: initdata.players.drain(..).map(|x| Arc::new(Mutex::new(x))).collect(),
        grid: Grid::gen_cell_auto(MAPDIM.0, MAPDIM.1, initdata.seed),
    });

    let gdc = gamedata.clone();
    let runningstatebool = Arc::new(atomic::AtomicBool::new(true));
    let rsbc = runningstatebool.clone();
    let (sender, recver) = mpsc::channel();
    let handle = thread::spawn(move || {
        client_netloop::netloop(upstream, gdc, pid, rsbc, recver)
    });
    let texture_creator = canvas.texture_creator();
    let gs = gamestate::ClientGamestate {
        handle,
        runningstate: runningstatebool,
        sdl: gamestate::Sdlstate {
            ctx: sdl_context,
            vid: video_subsystem,
            pump: Mutex::new(event_pump),
            texture_table: crate::render::texture_table::TextureTable::init(&texture_creator),
            canv: Mutex::new(canvas),
        },
        pid,
        gamedata,
        sender
    };
    
    let mut controller = Controller::new();
    let start = Instant::now();
    let mut i = 0;
    'running: loop {
        let now = Instant::now();
        let gametime = now.duration_since(start);
        i = (i + 1) % 255;
        
        if !controller.control(&gs.sdl.pump, gametime, gs.gamedata.clone(), gs.pid, &gs.sender) {
            gs.runningstate.store(false, atomic::Ordering::Relaxed);
            break 'running;
        }
        
        // println!("{:?}", gs.gamedata.players[gs.pid].lock().unwrap());
        // The rest of the game loop goes here...

        gs.render();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMERATE));
    }

    (move || {gs.handle.join().unwrap().unwrap();})();

    return Ok(());
}
