use serde::{Serialize, Deserialize};
use std::time::Duration;

const CIRCLE: [(isize, isize); 4]  = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Dir {
    WEST = 0, NORTH = 1, EAST = 2, SOUTH = 3, CRASH = 4
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Sequitur {
    move_timeout: Duration,
    move_pattern_index: usize,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Polandball {
    move_timeout: Duration,
    crash_timeout: Duration,
    move_direction: Dir,
}

impl Sequitur {
    pub fn new() -> Self {
        return Self {
            move_timeout: Duration::from_millis(500),
            move_pattern_index: 0,
        }
    }
}

impl Polandball {
    pub fn new() -> Self {
        return Self {
            crash_timeout: Duration::from_millis(1000),
            move_timeout: Duration::from_millis(200),
            move_direction: Dir::CRASH,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EnemyType {
    Sequitur(Sequitur),
    Polandball(Polandball)
}

impl EnemyType {
    pub fn drawindex(&self) -> i32 {
        match self {
            EnemyType::Sequitur(_sequitur) => 0,
            EnemyType::Polandball(_polska) => 1,
        }
    }
    pub fn move_timeout(&self) -> Duration {
        match self {
            EnemyType::Sequitur(sequitur) => sequitur.move_timeout,
            EnemyType::Polandball(polska) => {
                if Dir::CRASH == polska.move_direction {
                    polska.crash_timeout
                } else {
                    polska.move_timeout
                }
            }
        }
    }
    pub fn move_pattern(&mut self) -> (isize, isize) {
        match self {
            EnemyType::Sequitur(ref mut sequitur) => {
                let rv = CIRCLE[sequitur.move_pattern_index];
                sequitur.move_pattern_index = (sequitur.move_pattern_index + 1) % CIRCLE.len();
                rv
            }
            EnemyType::Polandball(ref mut polska) => {
                if polska.move_direction == Dir::CRASH {
                    polska.move_direction = match rand::random::<u8>() % 4 {
                        0 => Dir::WEST,
                        1 => Dir::NORTH,
                        2 => Dir::EAST,
                        3 => Dir::SOUTH,
                        _ => unreachable!()
                    };
                }
                CIRCLE[polska.move_direction as usize]
            }
        }
    }
    pub fn crash(&mut self) {
        match self {
            EnemyType::Polandball(polandball) => {
                polandball.move_direction = Dir::CRASH;
            }
            _ => {}
        }
    }
    pub fn render_dims(&self) -> (u32, u32) {
        match self {
            _ => (1, 1)
        }
    }
    pub fn sequitur() -> Self {
        EnemyType::Sequitur(Sequitur::new())
    }
    pub fn polandball() -> Self {
        EnemyType::Polandball(Polandball::new())
    }
}
