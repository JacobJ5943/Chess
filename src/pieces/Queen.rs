use crate::piece_types::PieceColor;
use crate::piece_types::QuickPiece;
use crate::pieces::PieceMove;

pub struct Queen {
    pos_x: usize,
    pos_y: usize,
    piece_color: PieceColor,
}

impl Queen {
    pub fn new(pos_x: usize, pos_y: usize, piece_color: PieceColor) -> Queen {
        Queen {
            pos_x,
            pos_y,
            piece_color,
        }
    }
}

impl PieceMove for Queen {
    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        true
    }

    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        // This will need access to the board of quick pieces as well.
        Vec::new()
    }
}
