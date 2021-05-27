pub mod board;
pub mod game;
pub mod parser;
pub mod piece_types;
pub mod pieces;

fn main() {
    game::play_game_cli();
}
