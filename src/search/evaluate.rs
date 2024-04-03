use crate::search::move_picker::ScoredMove;
use crate::search::node::Node;

static WEIGHTS: &[f32] = &[
    1.0, // Kichwa
    1.0, 1.0, 1.0, // Opponent's nyumba
    1.0, // Player's nyumba
    1.0, 1.0, 1.0, // Kichwa
    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
];

pub fn evaluate(node: Node, _legal_moves: &[ScoredMove]) -> f32 {
    let mut score = 0.0;

    // Material evaluation
    for (i, weight) in WEIGHTS.iter().enumerate().take(16) {
        score += weight * node.game.current_player.mashumo.get_seeds(i as i8) as f32;
        score -= weight * node.game.other_player.mashumo.get_seeds(i as i8) as f32;
    }

    score
}
