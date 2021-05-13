use super::super::piece_types::{PieceColor, QuickPiece};
use super::{coord_on_board, check_if_piece_on_location};
use super::PieceMove;

pub struct Bishop {
    pos_x: usize,
    pos_y: usize,
    piece_color: PieceColor,
}

impl Bishop {

    pub fn new(pos_x: usize, pos_y: usize, piece_color: PieceColor) -> Bishop {
        Bishop {
            pos_x,
            pos_y,
            piece_color,
        }
    }

    fn is_move_diagonal(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool{
        let x_delta = usize::max(x_coord, self.pos_x) - usize::min(x_coord, self.pos_x);
        let y_delta = usize::max(y_coord, self.pos_y) - usize::min(y_coord, self.pos_y);
        // This is to cover the case where the piece isn't moving anywhere
        if x_delta == 0 {
            return false
        }

        x_delta !=0 && x_delta==y_delta
    }
    /// This function should return  true if there are any pieces in the path to the final location.
    /// This does NOT include the final location
    /// This function assumes that the path it's going to is a valid location for a bishop
    fn check_if_pieces_in_path(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        let mut x_step:isize = 1;
        let mut y_step:isize = 1;
        if x_coord < self.pos_x {
            x_step = -1;
        }
        if y_coord < self.pos_y {
            y_step = -1;
        }

        // @TODO This looks gross and needs a refactor
        // That -1 I believe is so that it's all positions up to the final one will need to double check
        let max = usize::max(x_coord, self.pos_x);
        let min = usize::min(x_coord, self.pos_x);
        for index in 1..(max - min) {
            let current_pos:(isize, isize) =  (self.pos_x as isize + (x_step * index as isize), self.pos_y as isize + (y_step * index as isize));

            let result = match quick_board.get(current_pos.0 as usize).unwrap().get(current_pos.1 as usize).unwrap() {
                QuickPiece::PIECE(color) => true,
                QuickPiece::KING(color) => true,
                QuickPiece::EMPTY => false
            };
            if result {
                return true
            }
        }

        false
    }

}

impl PieceMove for Bishop {

    // @TODO All the returns feel wrong
    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        if !coord_on_board(x_coord, y_coord, quick_board) {
            return false
        }

        if !self.is_move_diagonal(x_coord,y_coord,quick_board) {
            return false;
        }


        // @TODO This may not need to be this functions responsibility anymore
        // Check if there are pieces in the way

        // Checking if pieces are in the path must be done before
        if self.check_if_pieces_in_path(x_coord,y_coord,quick_board) {
            return false
        }

        let mut piece_on_location_result = true;
        // @TODO this check could be refactored out so there is less doubling of work
        if check_if_piece_on_location(x_coord,y_coord, quick_board) {
            let piece = quick_board.get(x_coord).unwrap().get(y_coord).unwrap();
            piece_on_location_result = match piece {
                QuickPiece::PIECE(color) => !(*color == self.piece_color),
                QuickPiece::KING(color) => !(*color == self.piece_color),
                QuickPiece::EMPTY => true// @TODO This I don't know  This case should never occur, but panic seems wrong
            };
        }
        piece_on_location_result

    }




    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        // This will need access to the board of quick pieces as well.
        Vec::new()
    }
}
