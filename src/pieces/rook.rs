use super::super::piece_types::{PieceColor, QuickPiece};
use super::PieceMove;
use crate::pieces::movement;
use crate::pieces::{check_if_piece_on_location, coord_on_board};

#[derive(Copy, Clone, Debug)]
pub struct Rook {
    pos_x: usize,
    pos_y: usize,
    piece_color: PieceColor,
    has_moved: bool,
}

impl Rook {
    pub fn new(pos_x: usize, pos_y: usize, piece_color: PieceColor) -> Rook {
        Rook {
            pos_x,
            pos_y,
            piece_color,
            has_moved: false,
        }
    }

    pub fn get_has_moved(&self) -> bool {
        self.has_moved
    }
    pub fn set_has_moved(&mut self, value: bool) {
        self.has_moved = value;
    }
}

impl PieceMove for Rook {
    fn get_pos(&self) -> (usize, usize) {
        (self.pos_x, self.pos_y)
    }
    fn set_pos(&mut self, x_coord: usize, y_coord: usize) {
        self.pos_x = x_coord;
        self.pos_y = y_coord;
    }
    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        if !coord_on_board(x_coord, y_coord, quick_board) {
            return false;
        }

        if !movement::is_move_horizontal_vertical(
            self.pos_x,
            self.pos_y,
            x_coord,
            y_coord,
            quick_board,
        ) {
            return false;
        }

        // @TODO This may not need to be this functions responsibility anymore
        // Check if there are pieces in the way

        // Checking if pieces are in the path must be done before
        if movement::check_if_pieces_in_path_horizontal_vertical(
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

    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        let mut moves_vector: Vec<(usize, usize)> = Vec::new();

        let right = ((self.pos_x + 1)..8);
        let left = (0..self.pos_x).rev();
        let up = (self.pos_y + 1)..8;
        let down = (0..self.pos_y).rev();

        for coord in right {
            moves_vector.push((coord, self.pos_y))
        }
        for coord in left {
            moves_vector.push((coord, self.pos_y))
        }
        for coord in up {
            moves_vector.push((self.pos_x, coord))
        }
        for coord in down {
            moves_vector.push((self.pos_x, coord))
        }

        moves_vector
    }
}
