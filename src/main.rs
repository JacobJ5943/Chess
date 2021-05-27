pub mod board;
pub mod game;
pub mod parser;
pub mod piece_types;
pub mod pieces;

fn main() {
    println!("{}", String::from("P").as_str() == "P")
    // game::play_game_cli();
}
