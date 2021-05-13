pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use super::piece_types;
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
