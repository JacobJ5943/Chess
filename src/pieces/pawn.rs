use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::check_if_piece_on_location;
use crate::pieces::coord_on_board;
use crate::pieces::PieceMove;
use std::cmp::max;

#[derive(Copy, Clone)]
pub struct Pawn {
    pos_x: usize,
    pos_y: usize,
    piece_color: PieceColor,
    starting_y_position: usize,
}

impl Pawn {
    pub fn new(pos_x: usize, pos_y: usize, piece_color: PieceColor) -> Pawn {
        /*
        let starting_y_position = match piece_color {
            PieceColor::WHITE => 1,
            PieceColor::BLACK => 6,
        };
        */
        let starting_y_position = pos_y;
        Pawn {
            pos_x,
            pos_y,
            piece_color,
            starting_y_position,
        }
    }
    fn is_moving_forward(&self, y_coord: usize) -> bool {
        let y_delta = y_coord as isize - self.pos_y as isize;
        match self.piece_color {
            PieceColor::WHITE => {
                if y_delta < 1 {
                    false
                } else {
                    true
                }
            }
            PieceColor::BLACK => {
                if y_delta > -1 {
                    false
                } else {
                    true
                }
            }
        }
    }
}
impl PieceMove for Pawn {
    fn get_pos(&self) -> (usize, usize) {
        (self.pos_x, self.pos_y)
    }
    fn set_pos(&mut self, x_coord: usize, y_coord: usize) {
        self.pos_x = x_coord;
        self.pos_y = y_coord;
    }
    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        // A pawn can move forward left, forward right, forward
        if !coord_on_board(x_coord, y_coord, quick_board) {
            return false;
        }

        if x_coord == self.pos_x && y_coord == self.pos_y {
            return false;
        }

        if !self.is_moving_forward(y_coord) {
            return false;
        }

        let x_delta = max(self.pos_x, x_coord) - usize::min(self.pos_x, x_coord);
        if x_delta > 1 {
            return false;
        }

        let y_delta = max(self.pos_y, y_coord) - usize::min(self.pos_y, y_coord);
        if y_delta == 2 {
            if usize::max(self.starting_y_position, y_coord)
                - usize::min(self.starting_y_position, y_coord)
                != 2
            {
                return false;
            }
        }
        if y_delta > 2 {
            return false;
        }

        let mut piece_on_location_result = true;
        // @TODO this check could be refactored out so there is less doubling of work
        if check_if_piece_on_location(x_coord, y_coord, quick_board) {
            let piece = quick_board.get(x_coord).unwrap().get(y_coord).unwrap();
            piece_on_location_result = match piece {
                QuickPiece::PIECE(color) => !(*color == self.piece_color),
                QuickPiece::KING(color) => !(*color == self.piece_color),
                QuickPiece::EMPTY => true,
            };
        }
        piece_on_location_result
    }

    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        // This will need access to the board of quick pieces as well.
        Vec::new()
    }
}
