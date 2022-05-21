use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct VoidOrifice {
    pub move_timeout: Duration,
}

impl VoidOrifice {
    pub fn new() -> Self {
        Self {move_timeout: Duration::from_millis(300)}
    }
}
