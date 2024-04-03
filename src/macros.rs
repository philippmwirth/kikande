/// Construct a bao_game game from a series of moves.
///
/// # Examples
///
/// Create a new game in the starting position:
/// ```
/// # use kikande::bao_game;
/// # use kikande::bao::game_builder::GameBuilder;
/// let game = bao_game!();
/// ```
///
/// Create a new game with moves:
/// ```
/// # use kikande::bao_game;
/// # use kikande::bao::game_builder::GameBuilder;
/// let game = bao_game!("7L", "5R");
/// ```
///
/// Creating a game with invalid moves will fail at compile time:
/// ```compile_fail
/// # use kikande::bao_game;
/// let game = bao_game!("7L", "8R"); // 8R is an invalid move after 7L
/// ```
#[macro_export]
macro_rules! bao_game (
    ($($m:expr),* $(,)?) => {{
        let builder = GameBuilder::default();
        $(
            let builder = builder.with_move_str($m).unwrap();
        )*
        builder.build()
    }};
);

#[cfg(test)]
mod tests {
    use crate::bao::game_builder::GameBuilder;

    #[test]
    fn test_bao_game_macro_new_game() {
        let game = bao_game!();
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
    fn test_bao_game_macro_with_moves() {
        let game = bao_game!("7L", "5R");
        assert_eq!(game.current_player.seeds, 21);
        assert_eq!(game.other_player.seeds, 21);
        assert_eq!(
            game.current_player.mashumo.mashumo,
            [0, 0, 0, 0, 7, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            game.other_player.mashumo.mashumo,
            [0, 0, 0, 0, 7, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
