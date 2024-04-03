//! Bao game state.

use crate::bao::direction::Direction;
use crate::bao::move_factory::MoveFactory;
use crate::bao::moves::Move;
use crate::bao::player::Player;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct Game {
    pub current_player: Player,
    pub other_player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            current_player: Player::new(),
            other_player: Player::new(),
        }
    }

    pub fn take_turn(&mut self, first_move: &Move) {
        // Do we have a capture move?
        let is_mtaji_turn = first_move.is_capture();

        let mut deq = Some(*first_move);
        while let Some(m) = deq {
            // Put the move on the board.
            let end_index = if m.is_capture() {
                self.capture(m.index, m.get_direction(), m.is_namua())
            } else {
                self.relay(m.index, m.get_direction(), m.is_namua(), is_mtaji_turn)
            };

            // Check if the game is over.
            if self.other_player.mashumo.bitboard == 0 {
                return;
            }

            // Check follow-up moves.
            let direction = m.get_direction();
            let move_factory = MoveFactory::new(self); // TODO: Fix weird self dependency.
            deq = move_factory.get_follow_up_move_at_index(end_index, direction, is_mtaji_turn);
        }

        // Swap players.
        std::mem::swap(&mut self.current_player, &mut self.other_player);
    }

    fn capture(&mut self, source_index: i8, direction: Direction, is_namua: bool) -> i8 {
        // If we're in the NAMUA phase and the move introduces a new seed, add it.
        let capture_index = 7 - source_index;
        if is_namua {
            match self.other_player.mashumo.get_shumo_opposite(capture_index) {
                Some(index) => self.current_player.mashumo.increment_seeds(index),
                None => panic!("Invalid capture index"), // TODO: Improve error handling.
            };
            self.current_player.seeds -= 1;
        }

        // Capture the seeds.
        let seeds = self.other_player.mashumo.get_and_empty_seeds(capture_index);
        if capture_index == 4 {
            // Nyumba capture!
            self.other_player.nyumba = false;
        }
        match direction {
            Direction::Clockwise => self.current_player.mashumo.sow_cw(0, seeds as usize),
            Direction::CounterClockwise => self.current_player.mashumo.sow_ccw(7, seeds as usize),
        }
    }

    fn relay(
        &mut self,
        source_index: i8,
        direction: Direction,
        is_namua: bool,
        is_mtaji_turn: bool,
    ) -> i8 {
        // If we're in the NAMUA phase and the move introduces a new seed, add it.
        if is_namua {
            self.current_player.mashumo.increment_seeds(source_index);
            self.current_player.seeds -= 1;
        }
        let mut seeds = self
            .current_player
            .mashumo
            .get_and_empty_seeds(source_index);

        if source_index == 4 {
            // Special case: Nyumba
            if is_namua {
                // Only move 2 seeds.
                let old_seeds = seeds;
                seeds = old_seeds.min(2);
                self.current_player
                    .mashumo
                    .set_seeds(source_index, old_seeds - seeds);
            } else if is_mtaji_turn {
                // Go on a safari!
                self.current_player.nyumba = false;
            }
        }

        match direction {
            Direction::Clockwise => {
                let (_, start_index) = self.current_player.mashumo.get_shumo_cw(2, source_index);
                self.current_player
                    .mashumo
                    .sow_cw(start_index, seeds as usize)
            }
            Direction::CounterClockwise => {
                let (_, start_index) = self.current_player.mashumo.get_shumo_ccw(2, source_index);
                self.current_player
                    .mashumo
                    .sow_ccw(start_index, seeds as usize)
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut lines = "".to_string();
        // Pretty print the two player boards.
        lines.push('\n');
        let other_player_nyumba = if self.other_player.nyumba {
            "[✓]"
        } else {
            "[✗]"
        };
        lines.push_str(&format!(
            "Player 2: seeds={:02}, nyumba={}\n",
            self.other_player.seeds, other_player_nyumba
        ));
        lines.push('\n');
        lines.push_str("      8 7 6 5 4 3 2 1\n");
        lines.push_str("    -------------------\n");
        lines.push_str("    | ");
        for i in 0..8 {
            // Other player outer row.
            lines.push_str(&format!("{} ", self.other_player.mashumo.get_seeds(8 + i)));
        }
        lines.push_str("|\n");
        lines.push_str("  R | ");
        for i in 0..8 {
            // Other player inner row.
            lines.push_str(&format!("{} ", self.other_player.mashumo.get_seeds(7 - i)));
        }
        lines.push_str("| L  \n");
        lines.push_str("  L | ");
        for i in 0..8 {
            // Current player inner row.
            lines.push_str(&format!("{} ", self.current_player.mashumo.get_seeds(i)));
        }
        lines.push_str("| R  \n");
        lines.push_str("    | ");
        for i in 0..8 {
            // Current player outer row.
            lines.push_str(&format!(
                "{} ",
                self.current_player.mashumo.get_seeds(15 - i)
            ));
        }
        lines.push_str("|\n");
        lines.push_str("    -------------------\n");
        lines.push_str("      1 2 3 4 5 6 7 8\n");
        lines.push('\n');
        let current_player_nyumba = if self.current_player.nyumba {
            "[✓]"
        } else {
            "[✗]"
        };
        lines.push_str(&format!(
            "Player 1: seeds={:02}, nyumba={}\n",
            self.current_player.seeds, current_player_nyumba
        ));
        lines.push('\n');

        write!(f, "{}", lines)
    }
}
