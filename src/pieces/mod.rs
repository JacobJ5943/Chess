pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use super::piece_types;
use crate::piece_types::QuickPiece;

// Calling it PieceMove because I don't want to bother with if Move is already a trait or something
// silly
pub trait PieceMove {
    fn can_move(
        &self,
        x_coord: usize,
        y_coord: usize,
        quick_board: &Vec<Vec<piece_types::QuickPiece>>,
    ) -> bool;
    fn moves_on_board(&self) -> Vec<(usize, usize)>;
}

pub fn coord_on_board(x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<piece_types::QuickPiece>>) -> bool {
    let y_length = match quick_board.get(0) {
        Some(row) => row.len(),
        None => return false // @TODO Look into if this is rusty or crusty
    };

    if x_coord >= quick_board.len() || y_coord >= y_length {
        return false
    }
    true
}

fn check_if_piece_on_location(x_coord:usize, y_coord:usize, quick_board:&Vec<Vec<piece_types::QuickPiece>>) -> bool {
    let piece = quick_board.get(x_coord).unwrap().get(y_coord).unwrap();
    match piece {
        piece_types::QuickPiece::PIECE(color) => true,
        piece_types::QuickPiece::KING(color) => true,
        _ => false
    }

}
