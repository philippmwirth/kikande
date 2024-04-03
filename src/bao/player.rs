//! Bao players.

use crate::bao::board::Mashumo;

#[derive(Clone, Debug)]
pub struct Player {
    pub mashumo: Mashumo,
    pub seeds: u8,
    pub nyumba: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            mashumo: Mashumo::new(),
            seeds: 22,
            nyumba: true,
        }
    }
}
