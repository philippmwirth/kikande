use crate::bao::game::Game;
use crate::bao::move_factory::MoveFactory;
use crate::bao::moves::Move;
use crate::error::{GameBuilderError, GameBuilderResult};
use regex::Regex;

#[derive(Debug, Default)]
pub struct GameBuilder {
    game: Game,
}

impl GameBuilder {
    pub fn new() -> GameBuilder {
        GameBuilder { game: Game::new() }
    }

    pub fn build(self) -> Game {
        self.game
    }

    pub fn with_move(mut self, m: Move) -> GameBuilderResult<GameBuilder> {
        let mut move_factory = MoveFactory::new(&self.game);
        let legal_moves = move_factory.get_legal_moves();
        if !legal_moves.contains(&m) {
            return Err(GameBuilderError::IllegalMove(format!("{}", m)));
        }
        self.game.take_turn(&m);
        Ok(self)
    }

    pub fn with_moves(mut self, moves: &[Move]) -> GameBuilderResult<GameBuilder> {
        for m in moves {
            self = self.with_move(*m)?;
        }
        Ok(self)
    }

    pub fn with_move_str(self, m: &str) -> GameBuilderResult<GameBuilder> {
        println!("{}", m);
        let move_factory = MoveFactory::new(&self.game);
        let m = move_factory.parse_move(m)?;
        self.with_move(m)
    }

    pub fn with_moves_str(mut self, moves: &str) -> GameBuilderResult<GameBuilder> {
        let re = Regex::new(r"\d+[LR]|[aAbB]\d+[LR]|\b\d\b").unwrap();
        for cap in re.captures_iter(moves) {
            self = self.with_move_str(&cap[0])?;
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_game_builder_get_build() {
        let game = GameBuilder::default().build();
        assert_eq!(game.current_player.seeds, 22);
        assert_eq!(game.other_player.seeds, 22);
        assert_eq!(
            game.current_player.mashumo.mashumo,
            [0, 0, 0, 0, 6, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            game.other_player.mashumo.mashumo,
            [0, 0, 0, 0, 6, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_game_builder_with_move() -> GameBuilderResult<()> {
        let m = Move::new(5, 2); // namua, left, no capture
        let game = GameBuilder::default().with_move(m)?.build();
        // Note: The players have switched after the first move.
        assert_eq!(game.current_player.seeds, 22);
        assert_eq!(game.other_player.seeds, 21);
        assert_eq!(
            game.current_player.mashumo.mashumo,
            [0, 0, 0, 0, 6, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            game.other_player.mashumo.mashumo,
            [0, 0, 1, 1, 7, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        Ok(())
    }

    #[test]
    fn test_game_builder_with_moves() -> GameBuilderResult<()> {
        let moves = [
            Move::new(5, 2), // namua, left, no capture
            Move::new(4, 7), // namua, right, capture
        ];
        let game = GameBuilder::default().with_moves(&moves)?.build();
        assert_eq!(game.current_player.seeds, 21);
        assert_eq!(game.other_player.seeds, 21);
        assert_eq!(
            game.current_player.mashumo.mashumo,
            [0, 0, 1, 0, 7, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            game.other_player.mashumo.mashumo,
            [0, 0, 0, 0, 7, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        Ok(())
    }

    #[test]
    fn test_game_builder_with_moves_str() -> GameBuilderResult<()> {
        let game = GameBuilder::default().with_moves_str("6L 5R")?.build();
        assert_eq!(game.current_player.seeds, 21);
        assert_eq!(game.other_player.seeds, 21);
        let game = GameBuilder::default().with_moves_str("6L 5R; 3L")?.build();
        assert_eq!(game.current_player.seeds, 21);
        assert_eq!(game.other_player.seeds, 20);
        let game = GameBuilder::default()
            .with_moves_str("7L 5R; 6L 5R; 2 6R; 1 5R; 3L 5R; 5R")?
            .build();
        assert_eq!(game.current_player.seeds, 17);
        assert_eq!(game.other_player.seeds, 16);
        Ok(())
    }
}
