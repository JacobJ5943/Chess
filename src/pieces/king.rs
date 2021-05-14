use super::super::piece_types::{PieceColor, QuickPiece};
use super::super::pieces::{check_if_piece_on_location, coord_on_board};
use super::PieceMove;

pub struct King {
    pos_x: usize,
    pos_y: usize,
    piece_color: PieceColor,
}

impl King {
    pub fn new(pos_x: usize, pos_y: usize, piece_color: PieceColor) -> King {
        King {
            pos_x,
            pos_y,
            piece_color,
        }
    }
}
impl PieceMove for King {
    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        if !coord_on_board(x_coord, y_coord, quick_board) {
            return false;
        }

        let x_delta = usize::max(x_coord, self.pos_x) - usize::min(x_coord, self.pos_x);
        let y_delta = usize::max(y_coord, self.pos_y) - usize::min(y_coord, self.pos_y);

        if x_delta > 1 || y_delta > 1 {
            return false;
        }

        if x_delta == 1 && y_delta == 1 {
            return false;
        }

        if x_delta == 0 && y_delta == 0 {
            return false;
        }

        let mut piece_on_location_result = true;
        // @TODO this check could be refactored out so there is less doubling of work
        if check_if_piece_on_location(x_coord, y_coord, quick_board) {
            let piece = quick_board.get(x_coord).unwrap().get(y_coord).unwrap();
            piece_on_location_result = match piece {
                QuickPiece::PIECE(color) => !(*color == self.piece_color),
                QuickPiece::KING(color) => !(*color == self.piece_color),
                QuickPiece::EMPTY => true, // @TODO This I don't know  This case should never occur, but panic seems wrong
            };
        }
        piece_on_location_result
    }

    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        // This will need access to the board of quick pieces as well.
        Vec::new()
    }
}
