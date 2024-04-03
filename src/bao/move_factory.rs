//! Generate or parse Bao moves.

use crate::bao::direction::Direction;
use crate::bao::game::Game;
use crate::bao::moves::{flags, Move};
use crate::error::{MoveFactoryError, MoveFactoryResult};

pub struct MoveFactory<'a> {
    game: &'a Game,
    moves: [Move; 32], // Move and value
    num_moves: usize,
}

impl<'a> MoveFactory<'a> {
    pub fn new(game: &'a Game) -> Self {
        MoveFactory {
            game,
            moves: [Move::default(); 32],
            num_moves: 0,
        }
    }

    /// TODO
    pub fn get_legal_moves(&mut self) -> &[Move] {
        self.num_moves = 0;
        match self.game.current_player.seeds {
            0 => self.get_legal_moves_mtaji(),
            _ => self.get_legal_moves_namua(),
        };

        &self.moves[..self.num_moves]
    }

    /// TODO
    // Note: This does not check if a move is legal.
    pub fn parse_move(&self, s: &str) -> MoveFactoryResult<Move> {
        match s.len() {
            1 => {
                // Always a capture and always namua. Get index and direction.
                // Examples: "1", "2", "7", "8".
                let index = s[0..1].parse::<i8>()?;
                match index {
                    1..=2 => Ok(Move::namua_capture_left(index - 1)),
                    7..=8 => Ok(Move::namua_capture_right(index - 1)),
                    _ => Err(MoveFactoryError::ParseInvalidIndex(index)),
                }
            }
            2 => {
                // Either a capture in the center or a non-capture. Always namua.
                // Examples: "5L", "5R", "3L", "3R".
                let index = s[0..1].parse::<i8>()?;
                if !(1..=8).contains(&index) {
                    return Err(MoveFactoryError::ParseInvalidIndex(index));
                }
                if self.game.other_player.mashumo.bitboard & (1 << (index - 1)) != 0 {
                    match &s[1..2] {
                        "L" => Ok(Move::namua_capture_left(index - 1)),
                        "R" => Ok(Move::namua_capture_right(index - 1)),
                        _ => Err(MoveFactoryError::ParseInvalidDirection(s[1..2].to_string())),
                    }
                } else {
                    match &s[1..2] {
                        "L" => Ok(Move::namua_relay_left(index - 1)),
                        "R" => Ok(Move::namua_relay_right(index - 1)),
                        _ => Err(MoveFactoryError::ParseInvalidDirection(s[1..2].to_string())),
                    }
                }
            }
            3 => {
                // Either a capture or a non-capture. Never namua.
                let _is_capture = false; // TODO: Implement capture detection.
                panic!("Not implemented");
            }
            _ => Err(MoveFactoryError::ParseInvalidLength(s.len())),
        }
    }

    /// TODO
    pub fn get_follow_up_move_at_index(
        &self,
        index: i8,
        direction: Direction,
        is_mtaji_turn: bool,
    ) -> Option<Move> {
        // Check if the current player is "kufa".
        if self.game.current_player.mashumo.get_seeds(index) < 2 {
            return None;
        }

        // Check if the current player has a capture move.
        let flag_direction = if direction == Direction::Clockwise {
            flags::DIRECTION_R
        } else {
            0
        };
        if is_mtaji_turn {
            if let Some(capture_index) = self.game.current_player.mashumo.get_shumo_opposite(index)
            {
                let capturable_seeds = self.game.other_player.mashumo.get_seeds(capture_index);
                if capturable_seeds > 0 {
                    let move_flags = match index {
                        0..=1 => flags::CAPTURE | flags::RELAY,
                        6..=7 => flags::CAPTURE | flags::RELAY | flags::DIRECTION_R,
                        _ => flags::CAPTURE | flag_direction | flags::RELAY,
                    };
                    return Some(Move {
                        index,
                        flags: move_flags,
                    });
                }
            }
        }

        // Don't allow relay sowing from the nyumba.
        if !is_mtaji_turn && index == 4 {
            return None;
        }

        // The current player has a relay move.
        Some(Move {
            index,
            flags: flag_direction | flags::RELAY, // Continue in the same direction.
        })
    }

    fn get_legal_moves_mtaji(&mut self) -> &[Move] {
        let captures = self.game.current_player.mashumo.bitboard
            & self.game.other_player.mashumo.bitboard.reverse_bits();

        if captures != 0 {
            for source_index in 0..16 {
                match self.game.current_player.mashumo.get_seeds(source_index) {
                    0..=1 => {}
                    seeds => {
                        let mut shumo = self
                            .game
                            .current_player
                            .mashumo
                            .get_shumo_cw(seeds as usize + 1, source_index);
                        if shumo.1 < 8 && shumo.0 > 0 && (captures & (1 << (7 - shumo.1))) != 0 {
                            self.moves[self.num_moves] = Move::mtaji_capture_right(source_index);
                            self.num_moves += 1;
                        }
                        shumo = self
                            .game
                            .current_player
                            .mashumo
                            .get_shumo_ccw(seeds as usize + 1, source_index);
                        if shumo.1 < 8 && shumo.0 > 0 && (captures & (1 << (7 - shumo.1))) != 0 {
                            self.moves[self.num_moves] = Move::mtaji_capture_left(source_index);
                            self.num_moves += 1;
                        }
                    }
                }
            }
            if self.num_moves > 0 {
                return &self.moves[..self.num_moves];
            }
        }

        for source_index in 0..16 {
            match self.game.current_player.mashumo.get_seeds(source_index) {
                0..=1 => {}
                _ => {
                    self.moves[self.num_moves] = Move::mtaji_relay_right(source_index);
                    self.num_moves += 1;
                    self.moves[self.num_moves] = Move::mtaji_relay_left(source_index);
                    self.num_moves += 1;
                }
            };
        }
        &self.moves[..self.num_moves]
    }

    fn get_legal_moves_namua(&mut self) -> &[Move] {
        let captures = self.game.current_player.mashumo.bitboard
            & self.game.other_player.mashumo.bitboard.reverse_bits();
        if captures != 0 {
            for bits in 0..8 {
                if (captures & (1 << bits)) != 0 {
                    let source_index = 7 - bits;
                    match bits {
                        0..=1 => {
                            self.moves[self.num_moves] = Move::namua_capture_right(source_index);
                            self.num_moves += 1;
                        }
                        6..=7 => {
                            self.moves[self.num_moves] = Move::namua_capture_left(source_index);
                            self.num_moves += 1;
                        }
                        _ => {
                            self.moves[self.num_moves] = Move::namua_capture_right(source_index);
                            self.num_moves += 1;
                            self.moves[self.num_moves] = Move::namua_capture_left(source_index);
                            self.num_moves += 1;
                        }
                    };
                }
            }
            return &self.moves[..self.num_moves];
        }

        let relays = self.game.current_player.mashumo.bitboard;
        if relays & !(1 << 3) == 0 && relays & (1 << 3) != 0 {
            // Only the 5th pit (index 4) is non-empty.
            self.moves[0] = Move::namua_relay_right(4);
            self.moves[1] = Move::namua_relay_left(4);
            self.num_moves = 2;
            return &self.moves[..self.num_moves];
        }

        for bits in 0..8 {
            if self.game.current_player.mashumo.bitboard & (1 << bits) != 0 && bits != 3 {
                let source_index = 7 - bits;
                self.moves[self.num_moves] = Move::namua_relay_right(source_index);
                self.num_moves += 1;
                self.moves[self.num_moves] = Move::namua_relay_left(source_index);
                self.num_moves += 1;
            }
        }
        &self.moves[..self.num_moves]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_starting_position() -> MoveFactoryResult<()> {
        let game = Game::new();
        let factory = MoveFactory::new(&game);
        // Single char moves.
        assert_eq!(factory.parse_move("1")?, Move::namua_capture_left(0));
        assert_eq!(factory.parse_move("2")?, Move::namua_capture_left(1));
        assert!(factory.parse_move("3").is_err());
        assert!(factory.parse_move("4").is_err());
        assert!(factory.parse_move("5").is_err());
        assert!(factory.parse_move("6").is_err());
        assert_eq!(factory.parse_move("7")?, Move::namua_capture_right(6));
        assert_eq!(factory.parse_move("8")?, Move::namua_capture_right(7));
        // Two char moves.
        assert_eq!(factory.parse_move("1L")?, Move::namua_relay_left(0));
        assert_eq!(factory.parse_move("1R")?, Move::namua_relay_right(0));
        assert_eq!(factory.parse_move("2L")?, Move::namua_capture_left(1));
        assert_eq!(factory.parse_move("2R")?, Move::namua_capture_right(1));
        assert_eq!(factory.parse_move("3L")?, Move::namua_capture_left(2));
        assert_eq!(factory.parse_move("3R")?, Move::namua_capture_right(2));
        assert_eq!(factory.parse_move("4L")?, Move::namua_capture_left(3));
        assert_eq!(factory.parse_move("4R")?, Move::namua_capture_right(3));
        assert_eq!(factory.parse_move("5L")?, Move::namua_relay_left(4));
        assert_eq!(factory.parse_move("5R")?, Move::namua_relay_right(4));
        assert_eq!(factory.parse_move("6L")?, Move::namua_relay_left(5));
        assert_eq!(factory.parse_move("6R")?, Move::namua_relay_right(5));
        assert_eq!(factory.parse_move("7L")?, Move::namua_relay_left(6));
        assert_eq!(factory.parse_move("7R")?, Move::namua_relay_right(6));
        assert_eq!(factory.parse_move("8L")?, Move::namua_relay_left(7));
        assert_eq!(factory.parse_move("8R")?, Move::namua_relay_right(7));
        Ok(())
    }

    #[test]
    fn test_parse_starting_position_after_6_l() -> MoveFactoryResult<()> {
        let mut game = Game::new();
        let factory = MoveFactory::new(&game);
        game.take_turn(&factory.parse_move("6L")?);
        let factory = MoveFactory::new(&game);
        // Single char moves.
        assert_eq!(factory.parse_move("1")?, Move::namua_capture_left(0));
        assert_eq!(factory.parse_move("2")?, Move::namua_capture_left(1));
        assert!(factory.parse_move("3").is_err());
        assert!(factory.parse_move("4").is_err());
        assert!(factory.parse_move("5").is_err());
        assert!(factory.parse_move("6").is_err());
        assert_eq!(factory.parse_move("7")?, Move::namua_capture_right(6));
        assert_eq!(factory.parse_move("8")?, Move::namua_capture_right(7));
        // Two char moves.
        assert_eq!(factory.parse_move("1L")?, Move::namua_relay_left(0));
        assert_eq!(factory.parse_move("1R")?, Move::namua_relay_right(0));
        assert_eq!(factory.parse_move("2L")?, Move::namua_capture_left(1));
        assert_eq!(factory.parse_move("2R")?, Move::namua_capture_right(1));
        assert_eq!(factory.parse_move("3L")?, Move::namua_relay_left(2));
        assert_eq!(factory.parse_move("3R")?, Move::namua_relay_right(2));
        assert_eq!(factory.parse_move("4L")?, Move::namua_capture_left(3));
        assert_eq!(factory.parse_move("4R")?, Move::namua_capture_right(3));
        assert_eq!(factory.parse_move("5L")?, Move::namua_capture_left(4));
        assert_eq!(factory.parse_move("5R")?, Move::namua_capture_right(4));
        assert_eq!(factory.parse_move("6L")?, Move::namua_capture_left(5));
        assert_eq!(factory.parse_move("6R")?, Move::namua_capture_right(5));
        assert_eq!(factory.parse_move("7L")?, Move::namua_relay_left(6));
        assert_eq!(factory.parse_move("7R")?, Move::namua_relay_right(6));
        assert_eq!(factory.parse_move("8L")?, Move::namua_relay_left(7));
        assert_eq!(factory.parse_move("8R")?, Move::namua_relay_right(7));
        Ok(())
    }

    #[test]
    fn test_parse_errors() {
        let game = Game::new();
        let factory = MoveFactory::new(&game);
        match factory.parse_move("") {
            Err(e) => assert_eq!(e.to_string(), "ParseInvalidLength: 0"),
            _ => panic!(),
        }
        match factory.parse_move("abcd") {
            Err(e) => assert_eq!(e.to_string(), "ParseInvalidLength: 4"),
            _ => panic!(),
        }
        match factory.parse_move("0") {
            Err(e) => assert_eq!(e.to_string(), "ParseInvalidIndex: 0"),
            _ => panic!(),
        }
        match factory.parse_move("9") {
            Err(e) => assert_eq!(e.to_string(), "ParseInvalidIndex: 9"),
            _ => panic!(),
        }
        match factory.parse_move("0L") {
            Err(e) => assert_eq!(e.to_string(), "ParseInvalidIndex: 0"),
            _ => panic!(),
        }
        match factory.parse_move("9L") {
            Err(e) => assert_eq!(e.to_string(), "ParseInvalidIndex: 9"),
            _ => panic!(),
        }
        match factory.parse_move("1X") {
            Err(e) => assert_eq!(e.to_string(), "ParseInvalidDirection: X"),
            _ => panic!(),
        }
        match factory.parse_move("X") {
            Err(e) => assert_eq!(e.to_string(), "ParseError: invalid digit found in string"),
            _ => panic!(),
        }
    }
}
