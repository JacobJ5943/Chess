use std::env;

pub mod board;
pub mod game;
pub mod gui_runner;
pub mod parser;
pub mod piece_types;
pub mod pieces;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args.get(1).unwrap().as_str() {
            "gui" => game::play_game_gui(),
            _ => game::play_game_cli(),
        };
    } else {
        game::play_game_cli();
    }
}
