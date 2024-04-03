use crate::bao::game::Game;
use crate::bao::pv::PVLine;

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_game(game: &Game) {
    println!("{}", game);
}

pub fn print_game_mirror(game: &Game) {
    let mut mirror = game.clone();
    let tmp = mirror.current_player.clone();
    mirror.current_player = mirror.other_player.clone();
    mirror.other_player = tmp;
    println!("{}", mirror);
}

pub fn print_pvlines(pvlines: &[PVLine]) {
    println!("Depth: {}", pvlines[0].moves.len());
    for pvline in pvlines.iter().take(pvlines.len().min(20)) {
        println!("{}", pvline);
    }
}
