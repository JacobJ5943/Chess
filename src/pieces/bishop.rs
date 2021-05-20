use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::movement;
use crate::pieces::PieceMove;
use crate::pieces::{check_if_piece_on_location, coord_on_board};

#[derive(Copy, Clone)]
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
}

impl PieceMove for Bishop {
    // @TODO All the returns feel wrong
    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        if !coord_on_board(x_coord, y_coord, quick_board) {
            return false;
        }

        if !movement::is_move_diagonal(self.pos_x, self.pos_y, x_coord, y_coord, quick_board) {
            return false;
        }

        // @TODO This may not need to be this functions responsibility anymore
        // Check if there are pieces in the way

        // Checking if pieces are in the path must be done before
        if movement::check_if_pieces_in_path_diag(
            self.pos_x,
            self.pos_y,
            x_coord,
            y_coord,
            quick_board,
        ) {
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

    // I'm sure there's a much better way to create this funciton
    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        let mut moves_vector: Vec<(usize, usize)> = Vec::new();
        let upper_right = ((self.pos_x + 1)..8).zip((self.pos_y + 1)..8);
        let lower_right = ((self.pos_x + 1)..8).zip((0..self.pos_y).rev());
        let upper_left = (0..self.pos_x).rev().zip(self.pos_y + 1..8);
        let lower_left = (0..self.pos_x).rev().zip((0..self.pos_y).rev());

        for (left, right) in upper_right {
            moves_vector.push((left, right));
        }
        for (left, right) in lower_right {
            moves_vector.push((left, right));
        }
        for (left, right) in lower_left {
            moves_vector.push((left, right));
        }
        for (left, right) in upper_left {
            moves_vector.push((left, right));
        }

        moves_vector
    }

    fn get_pos(&self) -> (usize, usize) {
        (self.pos_x, self.pos_y)
    }

    fn set_pos(&mut self, x_coord: usize, y_coord: usize) {
        self.pos_x = x_coord;
        self.pos_y = y_coord;
    }
}
