use crate::board::Board;
use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::{PieceMove, AnyPiece};

use std::borrow::{Borrow, BorrowMut};
use crate::piece_types::QuickPiece::PIECE;

pub fn is_board_in_check(last_move: PieceColor, board: &Board) -> bool {
    let opposing_king = match last_move {
        PieceColor::WHITE => board.black_king_position,
        PieceColor::BLACK => board.white_king_position,
    };

    for piece in &board.live_white_pieces {
        if piece.can_move(opposing_king.0, opposing_king.1, &board.position_board) {
            return true;
        }
    }
    false
}


fn does_piece_check(
    piece: AnyPiece,
    opposing_king_x: usize,
    opposing_king_y: usize,
    quick_board: &Vec<Vec<QuickPiece>>,
) -> bool {
    piece.can_move(opposing_king_x, opposing_king_y, &quick_board)
}
