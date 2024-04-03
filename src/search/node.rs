use crate::bao::game::Game;
use crate::bao::moves::Move;
use crate::search::zobrist;

#[derive(Clone, Debug)]
pub struct Node {
    pub game: Game,
    pub zobrist: u64,
}

impl Default for Node {
    fn default() -> Self {
        Self::new(Game::default())
    }
}

impl Node {
    pub fn new(game: Game) -> Node {
        let zobrist = zobrist::zobrist(&game);
        Node { game, zobrist }
    }

    pub fn apply_move(&self, m: &Move) -> Node {
        let mut next_game = self.game.clone();
        next_game.take_turn(m);
        let zobrist = zobrist::zobrist(&next_game);

        Node {
            game: next_game,
            zobrist,
        }
    }
}
