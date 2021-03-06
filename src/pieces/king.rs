use super::super::piece_types::{PieceColor, QuickPiece};
use super::super::pieces::{check_if_piece_on_location, coord_on_board};
use super::PieceMove;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct King {
    pos_x: usize,
    pos_y: usize,
    piece_color: PieceColor,
    has_moved: bool,
}

impl King {
    pub fn new(pos_x: usize, pos_y: usize, piece_color: PieceColor) -> King {
        King {
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
impl PieceMove for King {
    fn get_pos(&self) -> (usize, usize) {
        (self.pos_x, self.pos_y)
    }

    fn set_pos(&mut self, x_coord: usize, y_coord: usize) {
        self.pos_x = x_coord;
        self.pos_y = y_coord;
    }

    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &[Vec<QuickPiece>]) -> bool {
        if !coord_on_board(x_coord, y_coord, quick_board) {
            return false;
        }

        let x_delta = usize::max(x_coord, self.pos_x) - usize::min(x_coord, self.pos_x);
        let y_delta = usize::max(y_coord, self.pos_y) - usize::min(y_coord, self.pos_y);

        if x_delta > 1 || y_delta > 1 {
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
        };
        piece_on_location_result
    }

    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        let mut possible_moves = Vec::new();
        if self.pos_y < 7 {
            if self.pos_x < 7 {
                (possible_moves.push((self.pos_x + 1, self.pos_y + 1)));
            }
            possible_moves.push((self.pos_x, self.pos_y + 1));
            if self.pos_x > 0 {
                possible_moves.push((self.pos_x - 1, self.pos_y + 1));
            }
        }

        if self.pos_x < 7 {
            possible_moves.push((self.pos_x + 1, self.pos_y));
        }

        if self.pos_x > 0 {
            possible_moves.push((self.pos_x - 1, self.pos_y));
        }

        if self.pos_y > 0 {
            if self.pos_x < 7 {
                (possible_moves.push((self.pos_x + 1, self.pos_y - 1)));
            }
            possible_moves.push((self.pos_x, self.pos_y - 1));
            if self.pos_x > 0 {
                possible_moves.push((self.pos_x - 1, self.pos_y - 1));
            }
        }
        possible_moves
    }
}
