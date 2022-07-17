use std::sync::{*, mpsc::*};
use std::collections::{VecDeque, BTreeMap, HashMap};
use std::time::{Duration, Instant};

use crate::gamestate::{Gamedata, InitializationData, EnemyDeltaEvent};
use crate::net::{pkt::PktPayload, *};
use crate::player::player::*;
use crate::map::grid::*;
use crate::entity::entity::{Entity, Etype};
use crate::enemy::enemy::Enemy;

const NET_HZ: u32 = 1000;

pub fn serveloop((mut stream, _addr): (std::net::TcpStream, std::net::SocketAddr), _gd: Arc<Gamedata>, sender: mpsc::Sender<PktPayload>, mut br: bus::BusReader<Arc<PktPayload>>, livelisteners: Arc<atomic::AtomicUsize>, index: usize) -> Result<(), String> {
    if let Ok(recvd) = br.recv() {
        let newpkt;
        if let PktPayload::Initial(initdata) = (*recvd).clone() {
            let mut newinitdata = initdata;
            //we actually populate the pid for this thread's peer
            newinitdata.pid = Some(index);
            newpkt = PktPayload::Initial(newinitdata);
        } else {
            panic!("Initialization packet was not a gamedata send?");
        }
        if let Err(s) = pkt::send_pkt(&mut stream, Arc::new(newpkt)) {
            if s.as_str() == "Fatal" {
                livelisteners.fetch_sub(1, atomic::Ordering::Relaxed);
                return Ok(());
            }
        }
    } else {
        panic!("Initialization send failed");
    }

    loop {
        let mut killflag = false;
        loop {
            match pkt::recv_pkt(&mut stream)  {
                Ok(recvd) => {
                    sender.send(recvd).unwrap();
                }
                Err(s) => {
                    if s.as_str() == "Fatal" {
                        livelisteners.fetch_sub(1, atomic::Ordering::Relaxed);
                        killflag = true;
                    }
                    break;
                }
            }
        }
        if killflag || livelisteners.load(atomic::Ordering::Relaxed) == 0 {
            break;
        }
        //if this doesn't run, assume for now it's just because we're nonblocking

        while let Ok(recvd) = br.try_recv() {
            if let Err(s) = pkt::send_pkt(&mut stream, recvd) {
                if s.as_str() == "Fatal" {
                    livelisteners.fetch_sub(1, atomic::Ordering::Relaxed);
                    killflag = true;
                }
            }
        }

        if killflag || livelisteners.load(atomic::Ordering::Relaxed) == 0 {
            break;
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / NET_HZ));
    }

    return Ok(());
}

pub fn gameloop() {
    let streams = servnet::initialize_server("127.0.0.1:9495".to_string());
    let mut spmc = bus::Bus::new(2048);

    let livelisteners = Arc::new(atomic::AtomicUsize::new(streams.len()));

    let (mpsc_tx, mpsc_rx) = channel();

    let mut mapseed = [0u8; 32];
    for i in 0..32 {
        mapseed[i] = rand::random::<u8>();
    }


    let (grid, mut playerlocs) = Grid::gen_cell_auto(MAPDIM.0, MAPDIM.1, mapseed, streams.len());
    
    let mut occupied = HashMap::new();
    for (i, loc) in playerlocs.iter().enumerate() {
        occupied.insert(*loc, (Etype::Player, i));
    }

    let mut enemy_tick_table: BTreeMap<Duration, Vec<usize>> = BTreeMap::new();

    let mut enemylocs = vec!();
    for i in 0..MAPDIM.0 {
        let mut randloc;
        loop {
            randloc = ((rand::random::<usize>() % MAPDIM.0) as isize,
                           (rand::random::<usize>() % MAPDIM.1) as isize);
            if grid.tiles[randloc.0 as usize + randloc.1 as usize* MAPDIM.0].passable {
                if !occupied.contains_key(&randloc) {
                    break;
                }
            }
        }
        occupied.insert(randloc, (Etype::Enemy, i));
        enemylocs.push(randloc);
    }

    let playarrs = playerlocs.drain(..).enumerate().map(|(i, x)| Arc::new(Mutex::new(Player::test_player(i, x)))).collect();

    let enemyarr = enemylocs.drain(..).enumerate().map(|(i, x)| Arc::new(Mutex::new(
                if rand::random::<bool>() {Enemy::test_enemy(i, x)} else {Enemy::fast_enemy(i, x)}
                ))).collect();

    enemy_tick_table.insert(Duration::from_millis(200), (0..MAPDIM.0).collect()); //moderate delay for starting
    let gd = Arc::new(Gamedata {
        players: playarrs,
        enemies: enemyarr,
        grid,
        occupation: Arc::new(RwLock::new(occupied)),
    });


    let handles = servnet::launch_server_workers(streams, gd.clone(), mpsc_tx, &mut spmc, livelisteners.clone());

    spmc.broadcast(Arc::new(PktPayload::Initial(InitializationData {players: gd.players.iter().map(|x| (*x.lock().unwrap()).clone()).collect(), enemies: gd.enemies.iter().map(|x| (*x.lock().unwrap()).clone()).collect(), seed: mapseed, pid: None})));

    let mut broadcasts_needed = VecDeque::new();
    let loop_start = Instant::now();
    loop {
        while let Ok(recvd) = mpsc_rx.try_recv() {
            match recvd {
                PktPayload::PlayerDelta(ref deltalist) => {
                    for delta in deltalist {
                        let mut deltaplayer = gd.players[delta.pid].lock().unwrap();
                        let dpp = deltaplayer.mut_pos();
                        let mut occupied = gd.occupation.write().unwrap();
                        occupied.remove(&dpp);
                        occupied.insert(delta.newpos, (Etype::Player, delta.pid));
                        dpp.0 = delta.newpos.0;
                        dpp.1 = delta.newpos.1;
                        //check that this position is valid, if not revert!?
                    }
                }
                PktPayload::EnemyDelta(_) => {
                    unreachable!(); // the server should never recieve enemy deltas
                }
                _ => {}
            }
            broadcasts_needed.push_back(recvd);
        }

        let now = Instant::now().duration_since(loop_start);

        let mut enemydeltas = vec!();
        let mut later_ticks = enemy_tick_table.split_off(&now);
        for (_tick, enemies) in enemy_tick_table {
            for enemyid in enemies {
                let mut enemy = gd.enemies[enemyid].lock().unwrap();
                let moveloc = enemy.enemy_type.move_pattern();

                let confirmedloc = enemy.mov(&gd, (Etype::Enemy, enemyid), moveloc);
                if let Some(newpos) = confirmedloc {
                    enemydeltas.push(EnemyDeltaEvent{eid: enemyid, newpos});
                } else {
                    enemy.enemy_type.crash();
                }

                enemy.mov_timeout(now);

                let newtick = *enemy.mut_mov_next();

                if let Some(ref mut vec) = later_ticks.get_mut(&newtick) {
                    vec.push(enemyid);
                } else {
                    later_ticks.insert(newtick, vec![enemyid]);
                }
            }
        }
        if !enemydeltas.is_empty() {
            broadcasts_needed.push_back(PktPayload::EnemyDelta(enemydeltas));
        }
        enemy_tick_table = later_ticks;

        while broadcasts_needed.len() > 0 {
            let frontel = broadcasts_needed.pop_front().unwrap();
            let tryfront = Arc::new(frontel);
            if let Err(_) = spmc.try_broadcast(tryfront.clone()) {
                broadcasts_needed.push_front(Arc::try_unwrap(tryfront).unwrap());
                break;
            }
        }

        if livelisteners.load(atomic::Ordering::Relaxed) == 0 {
            break;
        }

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / NET_HZ));
    }
    for handle in handles {
        handle.join().unwrap().unwrap()
    }
}
