use super::super::piece_types::{PieceColor, QuickPiece};
use super::PieceMove;

pub struct Pawn {
    pos_x: usize,
    pos_y: usize,
    piece_color: PieceColor,
}

impl Pawn {
    pub fn new(pos_x: usize, pos_y: usize, piece_color: PieceColor) -> Pawn {
        Pawn {
            pos_x,
            pos_y,
            piece_color,
        }
    }
}
impl PieceMove for Pawn {
    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        true
    }

    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        // This will need access to the board of quick pieces as well.
        Vec::new()
    }
}