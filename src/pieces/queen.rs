use crate::piece_types::PieceColor;
use crate::piece_types::QuickPiece;
use crate::pieces::movement;
use crate::pieces::PieceMove;
use crate::pieces::{check_if_piece_on_location, coord_on_board};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

        if x_coord == self.pos_x && y_coord == self.pos_y {
            return false;
        }

        if movement::is_move_diagonal(self.pos_x, self.pos_y, x_coord, y_coord, quick_board) {
            if movement::check_if_pieces_in_path_diag(
                self.pos_x,
                self.pos_y,
                x_coord,
                y_coord,
                quick_board,
            ) {
                return false;
            }
        } else if movement::is_move_horizontal_vertical(
            self.pos_x,
            self.pos_y,
            x_coord,
            y_coord,
            quick_board,
        ) {
            if movement::check_if_pieces_in_path_horizontal_vertical(
                self.pos_x,
                self.pos_y,
                x_coord,
                y_coord,
                quick_board,
            ) {
                return false;
            }
        } else {
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
        let mut moves_vector = Vec::new();
        let upper_right = ((self.pos_x + 1)..8).zip((self.pos_y + 1)..8);
        let lower_right = ((self.pos_x + 1)..8).zip((0..self.pos_y).rev());
        let upper_left = (0..self.pos_x).rev().zip(self.pos_y + 1..8);
        let lower_left = (0..self.pos_x).rev().zip((0..self.pos_y).rev());
        let right = (self.pos_x + 1)..8;
        let left = (0..self.pos_x).rev();
        let up = (self.pos_y + 1)..8;
        let down = (0..self.pos_y).rev();

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
