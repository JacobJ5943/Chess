use crate::game;
use crate::parser::{MoveTypes, ParsedMove};
use crate::piece_types::QuickPiece::PIECE;
use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::pieces::{AnyPiece, PieceMove};
use crate::{parser, pieces};
use std::any::Any;

pub struct Board {
    pub position_board: Vec<Vec<QuickPiece>>,
    pub live_white_pieces: Vec<AnyPiece>,
    pub live_black_pieces: Vec<AnyPiece>,
    pub white_king_position: (usize, usize),
    pub black_king_position: (usize, usize),
    pub last_move_color: PieceColor,
}

impl Board {
    pub fn new() -> Board {
        Board {
            position_board: Board::create_default_position_board(),
            live_white_pieces: Board::default_live_white_pieces(),
            live_black_pieces: Board::default_live_black_pieces(),
            white_king_position: Board::default_white_king_pos(),
            black_king_position: Board::default_black_king_pos(),
            last_move_color: PieceColor::BLACK,
        }
    }

    fn default_live_white_pieces() -> Vec<AnyPiece> {
        vec![
            AnyPiece::Rook(Rook::new(0, 0, PieceColor::WHITE)),
            AnyPiece::Rook(Rook::new(7, 0, PieceColor::WHITE)),
            AnyPiece::Knight(Knight::new(1, 0, PieceColor::WHITE)),
            AnyPiece::Knight(Knight::new(6, 0, PieceColor::WHITE)),
            AnyPiece::Bishop(Bishop::new(2, 0, PieceColor::WHITE)),
            AnyPiece::Bishop(Bishop::new(5, 0, PieceColor::WHITE)),
            AnyPiece::Queen(Queen::new(3, 0, PieceColor::WHITE)),
            AnyPiece::King(King::new(4, 0, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(0, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(1, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(2, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(3, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(4, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(5, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(6, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(7, 1, PieceColor::WHITE)),
        ]
    }

    fn default_live_black_pieces() -> Vec<AnyPiece> {
        vec![
            AnyPiece::Rook(Rook::new(0, 7, PieceColor::BLACK)),
            AnyPiece::Rook(Rook::new(7, 7, PieceColor::BLACK)),
            AnyPiece::Knight(Knight::new(1, 7, PieceColor::BLACK)),
            AnyPiece::Knight(Knight::new(6, 7, PieceColor::BLACK)),
            AnyPiece::Bishop(Bishop::new(2, 7, PieceColor::BLACK)),
            AnyPiece::Bishop(Bishop::new(5, 7, PieceColor::BLACK)),
            AnyPiece::Queen(Queen::new(3, 7, PieceColor::BLACK)),
            AnyPiece::King(King::new(4, 7, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(0, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(1, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(2, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(3, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(4, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(5, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(6, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(7, 6, PieceColor::BLACK)),
        ]
    }

    fn is_correct_piece_type(piece: &AnyPiece, compare_type: &str) -> bool {
        let compare_type = String::from(compare_type);
        match piece {
            AnyPiece::Knight(_) => compare_type == String::from("N"),
            AnyPiece::King(_) => compare_type == String::from("K"),
            AnyPiece::Queen(_) => compare_type == String::from("Q"),
            AnyPiece::Rook(_) => compare_type == String::from("R"),
            AnyPiece::Pawn(_) => compare_type == String::from("P"),
            AnyPiece::Bishop(_) => compare_type == String::from("B"),
        }
    }

    pub fn find_start_piece_from_move(
        &mut self,
        parsed_move: &parser::ParsedMove,
    ) -> Option<&mut AnyPiece> {
        let piece_list = match self.last_move_color {
            PieceColor::BLACK => &mut self.live_white_pieces,
            PieceColor::WHITE => &mut self.live_black_pieces,
        };

        let end_x: usize = parser::parse_coordinate(&parsed_move.end_coords.0);
        let end_y: usize = parser::parse_coordinate(&parsed_move.end_coords.1);
        let mut return_value = None;
        for piece in piece_list {
            // find a piece that can move to that location and is of the corret type
            // This move must not leave the player in check
            // This move must be valid.  A pawn can only move diagonal in a take or en passant
            if piece.can_move(end_x, end_y, &self.position_board)
                && Board::is_correct_piece_type(piece, &parsed_move.piece_char)
            {
                if &String::from("P") == &parsed_move.piece_char {
                    let delta_x =
                        usize::max(end_x, piece.get_pos().0) - usize::min(end_x, piece.get_pos().0);

                    let return_piece = match &parsed_move.move_type {
                        MoveTypes::Take => {
                            if delta_x == 1 {
                                return_value = Some(piece);
                            }
                        }
                        MoveTypes::Move => {
                            if delta_x == 0 {
                                return_value = Some(piece);
                            }
                        }
                        MoveTypes::Promote(piece_symbol) => {
                            match self.position_board.get(end_x).unwrap().get(end_y).unwrap() {
                                QuickPiece::EMPTY => {
                                    if delta_x == 0 {
                                        return_value = Some(piece);
                                    }
                                }
                                _ => {
                                    if delta_x == 1 {
                                        return_value = Some(piece);
                                    }
                                }
                            }
                        }
                        _ => (),
                    };
                } else {
                    return_value = Some(piece);
                }
            }
        }
        return_value
    }

    // @TODO ANother method that will have errors that need to be checked
    fn remove_piece_color(&mut self, x_coord: usize, y_coord: usize, color_to_remove: &PieceColor) {
        let piece_list = match color_to_remove {
            PieceColor::WHITE => {
                for (piece, index) in self
                    .live_white_pieces
                    .iter()
                    .zip(0..self.live_white_pieces.len())
                {
                    if piece.get_pos() == (x_coord, y_coord) {
                        self.live_white_pieces.remove(index);
                        return;
                    }
                }
            }
            PieceColor::BLACK => {
                for (piece, index) in self
                    .live_black_pieces
                    .iter()
                    .zip(0..self.live_black_pieces.len())
                {
                    if piece.get_pos() == (x_coord, y_coord) {
                        self.live_black_pieces.remove(index);
                        return;
                    }
                }
            }
        };
    }

    pub fn find_piece_color(
        &mut self,
        x_coord: usize,
        y_coord: usize,
        piece_color: &PieceColor,
    ) -> Option<&mut AnyPiece> {
        let piece_list = match piece_color {
            PieceColor::WHITE => &mut self.live_white_pieces,
            PieceColor::BLACK => &mut self.live_black_pieces,
        };

        for piece in piece_list {
            if piece.get_pos() == (x_coord, y_coord) {
                return Some(piece);
            }
        }
        None
    }

    pub fn can_any_piece_check_king(
        &mut self,
        x_coord: usize,
        y_coord: usize,
        king_color: &PieceColor,
    ) -> bool {
        let mut found_movement = false;
        match king_color {
            PieceColor::WHITE => {
                for piece in &self.live_black_pieces {
                    if piece.can_move(x_coord, y_coord, &self.position_board) {
                        found_movement = true;
                    }
                }
            }
            PieceColor::BLACK => {
                for piece in &self.live_white_pieces {
                    if piece.can_move(x_coord, y_coord, &self.position_board) {
                        found_movement = true;
                    }
                }
            }
        }
        found_movement
    }

    // @TODO What is this function again?
    fn create_default_pieces(color: &PieceColor) {
        // Create pawns
        //let mut live_pieces: Vec<Piece> = Vec::with_capacity(16); // TODO This should probably be a const config thing somewhere
        for _ in 0..8 {
            // Again I should do more config things in the future.

            //live_pieces.push(Piece::new(color));
        }
        for _ in 0..8 {
            //live_pieces.push(Piece::new(color));
        }
    }
    pub fn default_black_king_pos() -> (usize, usize) {
        (7, 4)
    }
    pub fn default_white_king_pos() -> (usize, usize) {
        (0, 4)
    }
    pub fn create_default_position_board() -> Vec<Vec<QuickPiece>> {
        // Cargo fmt is so gross
        vec![
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::KING(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::KING(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
        ]
    }

    fn move_in_quick_board(
        &mut self,
        moving_x: usize,
        moving_y: usize,
        moving_piece_color: &PieceColor,
        end_x: usize,
        end_y: usize,
    ) {
        let mut starting_piece = self
            .position_board
            .get_mut(moving_x)
            .unwrap()
            .remove(moving_y);
        self.position_board
            .get_mut(moving_x)
            .unwrap()
            .insert(moving_y, QuickPiece::EMPTY);
        self.position_board.get_mut(end_x).unwrap().remove(end_y);
        self.position_board
            .get_mut(end_x)
            .unwrap()
            .insert(end_y, starting_piece);
    }

    fn move_piece(
        &mut self,
        moving_x: usize,
        moving_y: usize,
        moving_piece_color: &PieceColor,
        end_x: usize,
        end_y: usize,
    ) {
        // @TODO This is where we must promote a pawn if it is a promotion move
        self.move_in_quick_board(moving_x, moving_y, moving_piece_color, end_x, end_y);

        let color_being_taken = match moving_piece_color {
            PieceColor::WHITE => PieceColor::BLACK,
            PieceColor::BLACK => PieceColor::WHITE,
        };

        self.remove_piece_color(end_x, end_y, &color_being_taken);

        // Set the piece's new position
        self.find_piece_color(moving_x, moving_y, moving_piece_color)
            .unwrap()
            .set_pos(end_x, end_y);
    }

    // Right now this I am assuming that this function is only used by my tests or after a move has been deemed valid
    // @TODO Maybe add if move says check or check mate make that check too
    pub fn play_move(&mut self, parsed_move: ParsedMove) {
        let current_move_color = PieceColor::opposite_color(&self.last_move_color);
        let mut moving_piece = self.find_start_piece_from_move(&parsed_move);
        let (moving_x, moving_y) = moving_piece.unwrap().get_pos();

        match &parsed_move.move_type {
            MoveTypes::Castle(king_end_x) => {
                self.castle_king(&current_move_color, *king_end_x);
            }
            _ => {
                let mut moving_piece = self.find_start_piece_from_move(&parsed_move);
                let (moving_x, moving_y) = moving_piece.unwrap().get_pos();
                let end_x = parser::parse_coordinate(&parsed_move.end_coords.0);
                let end_y = parser::parse_coordinate(&parsed_move.end_coords.1);

                if !game::will_move_be_in_check(
                    moving_x,
                    moving_y,
                    end_x,
                    end_y,
                    &current_move_color,
                    &current_move_color,
                    self,
                ) {
                    if let MoveTypes::Promote(piece_promote) = parsed_move.move_type {
                        self.move_piece(
                            moving_x,
                            moving_y,
                            &current_move_color,
                            end_x,
                            end_y,
                            Some(piece_promote.as_str()),
                        );
                    } else {
                        self.move_piece(
                            moving_x,
                            moving_y,
                            &current_move_color,
                            end_x,
                            end_y,
                            None,
                        );
                    }
                } else {
                    panic!(
                        "This move is in valid.  You are in check if you do the move {:?}",
                        parsed_move
                    );
                }
            }
        };
        self.last_move_color = PieceColor::opposite_color(&self.last_move_color);
    }
    pub fn can_castle_king(&mut self, king_color: &PieceColor, king_x_end: usize) -> bool {
        let mut can_castle = false;
        // 1. The king must not have moved
        let king_coords = match king_color {
            PieceColor::WHITE => self.white_king_position,
            PieceColor::BLACK => self.black_king_position,
        };

        let king_has_moved = match self.find_piece_color(king_coords.0, king_coords.1, king_color) {
            Some(king) => match king {
                AnyPiece::King(king) => king.get_has_moved(),
                _ => panic!("This should always be a king"),
            },
            None => panic!("This should always be a king"),
        };

        if !king_has_moved {
            // @TODO feels like I could replace this match with something cleaner.
            let check_rooks_and_empty = match king_x_end {
                6 => {
                    let rook = self.find_piece_color(7, king_coords.1, king_color);
                    let rook_has_not_moved = match rook {
                        Some(rook) => match rook {
                            AnyPiece::Rook(rook) => !rook.get_has_moved(),
                            _ => false,
                        },
                        None => false,
                    };
                    let no_pieces_in_path = self
                        .position_board
                        .get(5)
                        .unwrap()
                        .get(king_coords.1)
                        .unwrap()
                        == &QuickPiece::EMPTY
                        && self
                            .position_board
                            .get(6)
                            .unwrap()
                            .get(king_coords.1)
                            .unwrap()
                            == &QuickPiece::EMPTY;

                    rook_has_not_moved && no_pieces_in_path
                }
                2 => {
                    let rook = self.find_piece_color(0, king_coords.1, king_color);
                    let rook_has_not_moved = match rook {
                        Some(rook) => match rook {
                            AnyPiece::Rook(rook) => !rook.get_has_moved(),
                            _ => false,
                        },
                        None => false,
                    };
                    let no_pieces_in_path = self
                        .position_board
                        .get(2)
                        .unwrap()
                        .get(king_coords.1)
                        .unwrap()
                        == &QuickPiece::EMPTY
                        && self
                            .position_board
                            .get(3)
                            .unwrap()
                            .get(king_coords.1)
                            .unwrap()
                            == &QuickPiece::EMPTY;
                    rook_has_not_moved && no_pieces_in_path
                }
                _ => false,
            };
            if check_rooks_and_empty {
                // Then check if the king would be in check for those give positions
                can_castle = match king_x_end {
                    6 => {
                        let space1_check = !game::will_move_be_in_check(
                            king_coords.0,
                            king_coords.1,
                            king_x_end,
                            king_coords.1,
                            king_color,
                            king_color,
                            self,
                        );
                        let space2_check = !game::will_move_be_in_check(
                            king_coords.0,
                            king_coords.1,
                            5,
                            king_coords.1,
                            king_color,
                            king_color,
                            self,
                        );
                        space1_check && space2_check
                    }
                    2 => {
                        let space1_check = !game::will_move_be_in_check(
                            king_coords.0,
                            king_coords.1,
                            king_x_end,
                            king_coords.1,
                            king_color,
                            king_color,
                            self,
                        );
                        let space2_check = !game::will_move_be_in_check(
                            king_coords.0,
                            king_coords.1,
                            3,
                            king_coords.1,
                            king_color,
                            king_color,
                            self,
                        );
                        space1_check && space2_check
                    }
                    _ => false, // @TODO THis may be a mistake
                }
            }
        }

        can_castle
    }

    fn castle_king(&mut self, king_color: &PieceColor, king_x_end: usize) -> bool {
        if self.can_castle_king(king_color, king_x_end) {
            // Move the king to king_x_end
            let (king_x, king_y) = match king_color {
                PieceColor::WHITE => self.white_king_position,
                PieceColor::BLACK => self.black_king_position,
            };
            self.move_piece(king_x, king_y, king_color, king_x_end, king_y, None);
            if king_x_end == 6 {
                self.move_piece(7, king_y, king_color, 5, king_y, None);
            } else {
                self.move_piece(0, king_y, king_color, 3, king_y, None);
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::piece_types::PieceColor;
    use crate::pieces::king::King;
    use crate::pieces::rook::Rook;

    #[test]
    fn test_can_castle_white_only_no_moves() {
        let rook = Rook::new(0, 0, PieceColor::WHITE);
        let king = King::new(0, 4, PieceColor::WHITE);
    }
}
