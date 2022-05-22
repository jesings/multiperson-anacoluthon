use serde::{Serialize, Deserialize};
use super::void_orifice::*;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Class {
    VoidOrifice(VoidOrifice)
}

impl Class {
    pub fn move_timeout(&self) -> Duration {
        match self {
            Class::VoidOrifice(void_orifice) => void_orifice.move_timeout,
        }
    }
    
    pub fn void_orifice() -> Self {
        Class::VoidOrifice(VoidOrifice::new())
    }
}
