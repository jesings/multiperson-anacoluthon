use serde::{Serialize, Deserialize};
use std::time::Duration;

const CIRCLE: [(isize, isize); 4]  = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Sequitur {
    move_timeout: Duration,
    move_pattern_index: usize,
}

impl Sequitur {
    pub fn new() -> Self {
        return Self {
            move_timeout: Duration::from_millis(500),
            move_pattern_index: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum EnemyType {
    Sequitur(Sequitur)
}

impl EnemyType {
    pub fn move_timeout(&self) -> Duration {
        match self {
            EnemyType::Sequitur(sequitur) => sequitur.move_timeout,
        }
    }
    pub fn move_pattern(&mut self) -> (isize, isize) {
        match self {
            EnemyType::Sequitur(ref mut sequitur) => {
                let rv = CIRCLE[sequitur.move_pattern_index];
                sequitur.move_pattern_index = (sequitur.move_pattern_index + 1) % CIRCLE.len();
                rv
            }
        }
    }
    pub fn sequitur() -> Self {
        EnemyType::Sequitur(Sequitur::new())
    }
}
