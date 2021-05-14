use crate::board::Board;
use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::PieceMove;
use std::borrow::{Borrow, BorrowMut};

pub fn is_board_in_check(last_move: PieceColor, board: &Board) -> bool {
    println!(
        "BLACK POS{},{}",
        board.black_king_position.0, board.black_king_position.1
    );
    println!(
        "WHITE POS{},{}",
        board.white_king_position.0, board.white_king_position.1
    );
    let opposing_king = match last_move {
        PieceColor::WHITE => board.black_king_position,
        PieceColor::BLACK => board.white_king_position,
    };
    println!("OPPOSING:{:?}", opposing_king);
    for piece in &board.live_white_pieces {
        if piece.can_move(opposing_king.0, opposing_king.1, &board.position_board) {
            println!("HOW DID I GET HERE!!!");
            // I don't know why I can't send &piece to does piece check
            //if does_piece_check(&piece, opposing_king.0, opposing_king.1, &board.position_board) {
            return true;
        }
    }
    false
}

fn does_piece_check(
    piece: &Box<dyn PieceMove>,
    opposing_king_x: usize,
    opposing_king_y: usize,
    quick_board: &Vec<Vec<QuickPiece>>,
) -> bool {
    piece.can_move(opposing_king_x, opposing_king_y, &quick_board)
}
