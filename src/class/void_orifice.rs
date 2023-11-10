use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct VoidOrifice {
    pub move_timeout: Duration,
    pub skill_timeout: [Duration; 4],
    pub directional_skill: [bool; 4],
    pub skill_next: [Duration; 4],
}

impl VoidOrifice {
    pub fn new() -> Self {
        Self {move_timeout: Duration::from_millis(300),
              skill_timeout: [Duration::from_millis(300); 4],
              directional_skill: [true; 4],
              skill_next: [Duration::from_millis(0); 4],
        }
    }
}
