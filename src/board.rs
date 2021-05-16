use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces;
use crate::pieces::{AnyPiece, PieceMove};

pub struct Board {
    pub position_board: Vec<Vec<QuickPiece>>,
    pub live_white_pieces: Vec<AnyPiece>,
    pub live_black_pieces: Vec<AnyPiece>,
    pub white_king_position: (usize, usize),
    pub black_king_position: (usize, usize),
}

impl Board {
    pub fn new() -> Board {
        Board {
            position_board: Board::create_default_position_board(),
            live_white_pieces: Vec::new(),
            live_black_pieces: Vec::new(),
            white_king_position: Board::default_white_king_pos(),
            black_king_position: Board::default_black_king_pos(),
        }
    }

    pub fn find_piece(
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
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::KING(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
            ],
            vec![
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
            ],
            vec![
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
            ],
            vec![
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
            ],
            vec![
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
            ],
            vec![
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::KING(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
        ]
    }
}
