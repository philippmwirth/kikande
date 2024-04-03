//! Iterative deepening negamax search with alpha-beta pruning.

use crate::bao::game::Game;
use crate::bao::moves::Move;
use crate::bao::pv::PVLine;
use crate::config::SearchConfig;
use crate::error::SearchResult;
use crate::search::evaluate;
use crate::search::move_picker::MovePicker;
use crate::search::node::Node;
use crate::search::timer::Timer;
use crate::search::transposition_table::{EntryType, TranspositionTable};
use std::sync::Arc;

use crate::display;
use std::sync::mpsc;

use std::thread;

/// Search for the best move.
///
/// # Arguments
///
/// * `config` - The search configuration.
/// * `node` - The root node to search from.
/// * `verbose` - Whether to display the search progress in the terminal.
///
/// # Returns
///
/// A principal variation line.
pub fn search(config: SearchConfig, game: Game, verbose: bool) -> SearchResult<PVLine> {
    // Initialize shared transposition table and channels.
    let tt = Arc::new(TranspositionTable::new());
    let (sender, receiver) = mpsc::channel();

    // Initialize timer.
    let timer = Timer::new(config.max_time_ms.unwrap_or(u32::MAX));

    // Start multi-threaded iterative deepening.
    for _ in 0..config.num_threads {
        let node = Node::new(game.clone());
        let sender = sender.clone();
        let timer = timer.clone();
        let tt = tt.clone();
        thread::spawn(move || -> SearchResult<()> {
            iterative_deepening(config.max_depth, node, sender, &timer, &tt)
        });
    }

    drop(sender); // Drop so threads can finish.

    // Main thread displays the board and PV lines.
    let mut pvlines: Vec<PVLine> = vec![];
    let mut current_search_depth = 0;
    while let Ok(pvline) = receiver.recv() {
        // Update search depth.
        current_search_depth = current_search_depth.max(pvline.get_depth());

        // Insert and sort.
        pvlines.push(pvline);
        pvlines.sort_by(|a, b| a.partial_cmp(b).expect("Failed to compare PVLine"));

        if verbose {
            // Display the top 3 PV lines.
            display::clear_terminal();
            display::print_game(&game);
            println!("{}", timer);
            display::print_pvlines(&pvlines);
        }
    }

    // Return the best PV line.
    Ok(pvlines.remove(0))
}

/// Iterative deepening negamax search.
///
/// # Arguments
///
/// * `max_depth` - The maximum depth to search.
/// * `node` - The root node to search from.
/// * `sender` - The channel to send the principal variation line.
/// * `timer` - The timer to check if time is up.
/// * `tt` - The transposition table to store results.
fn iterative_deepening(
    max_depth: u8,
    node: Node,
    sender: std::sync::mpsc::Sender<PVLine>,
    timer: &Timer,
    tt: &Arc<TranspositionTable>,
) -> SearchResult<()> {
    for depth in 1..max_depth + 1 {
        negamax(
            node.clone(),
            depth,
            f32::NEG_INFINITY,
            f32::INFINITY,
            timer,
            tt,
        )?;
        if timer.is_time_up() {
            break; // We can't guarantee the tree is fully searched.
        }
        let pvline = tt.get_pv_line(&node, depth);
        sender.send(pvline)?;
    }
    Ok(())
}

/// Negamax search with alpha-beta pruning.
///
/// # Arguments
///
/// * `node` - The current node to search from.
/// * `depth` - The maximum depth to search.
/// * `alpha` - The alpha value.
/// * `beta` - The beta value.
/// * `timer` - The timer to check if time is up.
/// * `tt` - The transposition table to store results.
fn negamax(
    node: Node,
    depth: u8,
    mut alpha: f32,
    mut beta: f32,
    timer: &Timer,
    tt: &Arc<TranspositionTable>,
) -> SearchResult<f32> {
    // Time's up, return immediately. Returning 0.0 does not affect the result.
    if timer.is_time_up() {
        return Ok(0.0);
    }

    let orig_alpha = alpha;

    // Probe tt
    let entry = tt.probe(node.zobrist);
    let mut tt_move = None;
    if let Some(entry) = entry {
        // Use tt entry if it's valid.
        if entry.depth >= depth {
            match entry.entry_type {
                EntryType::Exact => {
                    return Ok(entry.score);
                }
                EntryType::LowerBound => {
                    alpha = alpha.max(entry.score);
                }
                EntryType::UpperBound => {
                    beta = beta.min(entry.score);
                }
            }
            if alpha >= beta {
                return Ok(entry.score);
            }
        }
        // Search tt move first.
        tt_move = Some(entry.best_move);
    }

    // Generate possible moves.
    let mut picker = MovePicker::new();
    let legal_moves = picker.pick_moves(&node.game, tt_move);

    // Terminal node or max depth, evaluate and return.
    if legal_moves.is_empty() || node.game.current_player.mashumo.bitboard == 0 {
        return Ok(f32::NEG_INFINITY);
    } else if node.game.other_player.mashumo.bitboard == 0 {
        return Ok(f32::INFINITY);
    } else if depth == 0 {
        return Ok(evaluate::evaluate(node, legal_moves));
    }

    // Recursive search
    let mut value = f32::NEG_INFINITY;
    let mut best_move: Option<Move> = None;
    for m in legal_moves {
        let child = node.apply_move(&m.m);
        value = value.max(-negamax(child, depth - 1, -beta, -alpha, timer, tt)?);

        if value > alpha {
            alpha = value;
            best_move = Some(m.m);
        }

        if alpha >= beta {
            break; // Beta cut-off
        }
    }

    // Update tt.
    let mut tt_flag: EntryType = EntryType::Exact;
    if value <= orig_alpha {
        tt_flag = EntryType::UpperBound;
    } else if value >= beta {
        tt_flag = EntryType::LowerBound;
    }
    if let Some(best_move) = best_move {
        tt.insert(node.zobrist, &best_move, depth, value, tt_flag);
    }

    Ok(value)
}
