use crate::bao::game::Game;
use crate::bao::move_factory::MoveFactory;
use crate::bao::moves::Move;

#[derive(Copy, Clone)]
pub struct ScoredMove {
    pub m: Move,
    score: i8,
}

impl ScoredMove {
    pub fn new(m: Move, score: i8) -> ScoredMove {
        ScoredMove { m, score }
    }
}

impl Default for ScoredMove {
    fn default() -> ScoredMove {
        ScoredMove::new(Move { index: 0, flags: 0 }, 0)
    }
}

#[derive(Default)]
pub struct MovePicker {
    moves: [ScoredMove; 32], // Move and value
    num_moves: usize,
}

impl MovePicker {
    pub fn new() -> Self {
        MovePicker {
            moves: [ScoredMove::default(); 32],
            num_moves: 0,
        }
    }

    pub fn pick_moves(&mut self, game: &Game, tt_move: Option<Move>) -> &[ScoredMove] {
        // Generate legal moves.
        let mut move_factory = MoveFactory::new(game);
        let legal_moves = move_factory.get_legal_moves();
        self.num_moves = legal_moves.len();
        for (i, legal_move) in legal_moves.iter().enumerate().take(self.num_moves) {
            self.moves[i].m = *legal_move;
        }

        // Sort by value.
        self.score(game);
        self.moves[..self.num_moves].sort_by(|a, b| b.score.cmp(&a.score));

        // Search tt move first.
        self.insert_tt_move(tt_move);

        &self.moves[..self.num_moves]
    }

    fn insert_tt_move(&mut self, tt_move: Option<Move>) {
        if let Some(tt_move) = tt_move {
            for i in 0..self.num_moves {
                if self.moves[i].m == tt_move {
                    self.moves[0..i + 1].rotate_right(1);
                    break;
                }
            }
        }
    }

    fn score(&mut self, game: &Game) {
        for m in &mut self.moves[..self.num_moves] {
            if m.m.is_capture() {
                let capture_index = 7 - m.m.index;
                // Bonus for protecting own seeds
                m.score += game.current_player.mashumo.get_seeds(m.m.index) as i8;
                // Bonus for protecting nyumba
                m.score += if capture_index == 3 && game.current_player.nyumba {
                    2
                } else {
                    0
                };
            }
        }
    }
}
