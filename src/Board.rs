use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces;

struct Board {
    position_board: Vec<Vec<QuickPiece>>,
    live_white_pieces: Vec<Box<dyn pieces::PieceMove>>,
    live_black_pieces: Vec<Box<dyn pieces::PieceMove>>,
}

impl Board {
    fn new() -> Board {
        Board {
            position_board: Board::create_default_position_board(),
            live_white_pieces: Vec::new(),
            live_black_pieces: Vec::new(),
        }
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
