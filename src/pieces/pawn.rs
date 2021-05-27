use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::coord_on_board;
use crate::pieces::PieceMove;
use std::cmp::max;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Pawn {
    pos_x: usize,
    pos_y: usize,
    piece_color: PieceColor,
    starting_y_position: usize,
}

impl Pawn {
    pub fn new(pos_x: usize, pos_y: usize, piece_color: PieceColor) -> Pawn {
        let starting_y_position = pos_y;
        Pawn {
            pos_x,
            pos_y,
            piece_color,
            starting_y_position,
        }
    }
    fn has_moved(&self) -> bool {
        match self.piece_color {
            PieceColor::WHITE => self.pos_y != 1,
            PieceColor::BLACK => self.pos_y != 6,
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
            let piece = quick_board
                .get(x_coord)
                .unwrap()
                .get(usize::max(y_coord, self.pos_y) - 1)
                .unwrap();
            match piece {
                QuickPiece::PIECE(_) => return false,
                QuickPiece::KING(_) => return false,
                QuickPiece::EMPTY => (),
            };
        }
        if y_delta > 2 {
            return false;
        }

        let mut piece_on_location_result = false;
        // @TODO this check could be refactored out so there is less doubling of work
        if x_delta == 1 && y_delta == 1 {
            let piece = quick_board.get(x_coord).unwrap().get(y_coord).unwrap();
            piece_on_location_result = match piece {
                QuickPiece::PIECE(color) => *color != self.piece_color,
                QuickPiece::KING(color) => *color != self.piece_color,
                QuickPiece::EMPTY => true,
            };
        }
        if x_delta == 0 {
            let piece = quick_board.get(x_coord).unwrap().get(y_coord).unwrap();
            piece_on_location_result = match piece {
                QuickPiece::PIECE(_) => false,
                QuickPiece::KING(_) => false,
                QuickPiece::EMPTY => true,
            };
        }
        piece_on_location_result
    }

    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        let mut possible_moves = Vec::new();
        match self.piece_color {
            PieceColor::WHITE => {
                possible_moves.push((self.pos_x, self.pos_y + 1));

                if self.pos_x > 0 {
                    possible_moves.push((self.pos_x - 1, self.pos_y + 1));
                }

                if self.pos_x < 7 {
                    possible_moves.push((self.pos_x + 1, self.pos_y + 1));
                }

                if !self.has_moved() {
                    possible_moves.push((self.pos_x, self.pos_y + 2));
                }
            }
            PieceColor::BLACK => {
                possible_moves.push((self.pos_x, self.pos_y - 1));

                if self.pos_x > 0 {
                    possible_moves.push((self.pos_x - 1, self.pos_y - 1));
                }

                if self.pos_x < 7 {
                    possible_moves.push((self.pos_x + 1, self.pos_y - 1));
                }

                if !self.has_moved() {
                    possible_moves.push((self.pos_x, self.pos_y - 2));
                }
            }
        };

        possible_moves
    }
}
