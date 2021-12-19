use std::sync::*;
use crate::gamestate::*;
use super::class::*;

pub struct VoidOrifice {
    // ._. <- bob the funny chungus
}

impl Classlike for VoidOrifice {
    fn mov(&self, gamestate: Arc<ClientGamestate>, dir: (usize, usize)) {
        match dir {
            (0, c) => false,
            (r, _) => false,
        };
    }
}
