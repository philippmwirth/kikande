use crate::bao::game::Game;

#[inline]
pub fn zobrist(game: &Game) -> u64 {
    let mut hash = game.current_player.mashumo.zobrist;
    hash ^= game.other_player.mashumo.zobrist.reverse_bits();
    hash
}
