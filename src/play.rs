//! Play a game against the computer.

use crate::bao::game::Game;
use crate::bao::move_factory::MoveFactory;
use crate::bao::moves::Move;
use crate::config::SearchConfig;
use crate::display;
use crate::search::negamax;
use std::io;

/// Play a game against the computer.
///
/// # Arguments
///
/// * `difficulty` - The difficulty level (1-10).
pub fn play(difficulty: u8) {
    let config = SearchConfig::new_from_difficulty(difficulty);

    // Start a new game.
    let mut game = Game::default();
    loop {
        // Display current board state.
        display::clear_terminal();
        display::print_game(&game);

        // Get the player's move.
        game = match player_turn(&game) {
            Some(game) => game,
            None => break, // Game over.
        };

        // Display current board state.
        display::clear_terminal();
        display::print_game_mirror(&game);

        game = match computer_turn(&game, &config) {
            Some(game) => game,
            None => break, // Game over.
        }
    }
}

fn player_turn(game: &Game) -> Option<Game> {
    // Get all legal moves.
    let mut factory = MoveFactory::new(game);
    let legal_moves = factory.get_legal_moves();
    if legal_moves.is_empty() {
        return None; // Game over.
    }

    // Prompt player for move.
    prompt_player_move(game, legal_moves)
}

fn prompt_player_move(game: &Game, legal_moves: &[Move]) -> Option<Game> {
    let mut next_game = None;
    while next_game.is_none() {
        // Prompt player for move.
        println!("Enter move: ");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }
        // Parse move.
        let factory = MoveFactory::new(game);
        let m = match factory.parse_move(&input) {
            Ok(m) => m,
            _ => continue,
        };
        // Check if move is legal.
        if !legal_moves.iter().any(|sm| *sm == m) {
            continue;
        }
        let mut game_after_move = game.clone();
        game_after_move.take_turn(&m);
        next_game = Some(game_after_move);
    }
    next_game
}

fn computer_turn(game: &Game, config: &SearchConfig) -> Option<Game> {
    // Create search node
    let pvline = match negamax::search(config.clone(), game.clone(), false) {
        Ok(pvline) => pvline,
        Err(_) => return None, // Game over.
    };

    // Show the computer's move.
    display::clear_terminal();
    display::print_game_mirror(game);
    display::print_pvlines(&[pvline.clone()]);
    println!("Your opponent will play: {}", pvline.moves.first()?);
    println!("Press enter to continue...");
    io::stdin()
        .read_line(&mut String::new())
        .expect("read error");

    // Play the move and return the next state.
    let next_move = pvline.moves.first().expect("No moves");
    let mut factory = MoveFactory::new(game);
    for m in factory.get_legal_moves() {
        if next_move == m {
            let mut next_game = game.clone();
            next_game.take_turn(m);
            return Some(next_game);
        }
    }
    None
}
