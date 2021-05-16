use crate::board::Board;
use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::{AnyPiece, PieceMove};

use crate::piece_types::QuickPiece::PIECE;
use std::borrow::{Borrow, BorrowMut};

// This function only checks the color opposing the last move.  This is because one cannot make a
// move that would put the player in check.  That means for this function to run it has already
// been checked that the king of the last_move color is not in check.
pub fn is_board_in_check(last_move: &PieceColor, board: &Board) -> bool {
    let opposing_king = match last_move {
        PieceColor::WHITE => board.black_king_position,
        PieceColor::BLACK => board.white_king_position,
    };
    match last_move {
        PieceColor::WHITE => {
            for piece in &board.live_white_pieces {
                if piece.can_move(opposing_king.0, opposing_king.1, &board.position_board) {
                    return true;
                }
            }
        }
        PieceColor::BLACK => {
            for piece in &board.live_black_pieces {
                if piece.can_move(opposing_king.0, opposing_king.1, &board.position_board) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn is_board_check_mate(last_move: &PieceColor, board: &mut Board) -> bool {
    let opposing_king = match last_move {
        PieceColor::WHITE => board.black_king_position,
        PieceColor::BLACK => board.white_king_position,
    };
    let opposing_king_color = match last_move {
        PieceColor::WHITE => PieceColor::BLACK,
        PieceColor::BLACK => PieceColor::WHITE,
    };

    if is_board_in_check(last_move, board) {
        let king = board
            .find_piece(opposing_king.0, opposing_king.1, &opposing_king_color)
            .unwrap()
            .clone();
        let possible_moves: Vec<(usize, usize)> = king.moves_on_board();
        let mut found_safe_spot = false;
        for test in possible_moves {
            if king.can_move(test.0, test.1, &board.position_board) {
                let result = !will_move_be_in_check(
                    opposing_king.0,
                    opposing_king.1,
                    test.0,
                    test.1,
                    &opposing_king_color,
                    &opposing_king_color,
                    board,
                );
                if result {
                    found_safe_spot = true
                }
            }
        }
        return !found_safe_spot;
    }

    false
}

/// While this function takes in a mutable board it will not return until the board is back to the original state
/// @TODO Think about where I want more input checks
/// @TODO this feels kind of long
pub fn will_move_be_in_check(
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
    last_move_color: &PieceColor,
    king_color_being_checked: &PieceColor,
    board: &mut Board,
) -> bool {
    assert!(0 <= x_start && x_start <= 7);
    assert!(0 <= y_start && y_start <= 7);
    assert!(0 <= y_end && y_end <= 7);
    assert!(0 <= x_end && x_end <= 7);

    let start_piece = board
        .position_board
        .get_mut(x_start)
        .unwrap()
        .remove(y_start);

    /*let last_move_color = match king_color_being_checked {
        PieceColor::WHITE => PieceColor::BLACK,
        PieceColor::BLACK => PieceColor::WHITE
    };*/

    let mut living_pieces = match king_color_being_checked {
        PieceColor::WHITE => &mut board.live_black_pieces,
        PieceColor::BLACK => &mut board.live_white_pieces,
    };

    board
        .position_board
        .get_mut(x_start)
        .unwrap()
        .insert(y_start, QuickPiece::EMPTY);
    let end_piece = board.position_board.get_mut(x_end).unwrap().remove(y_end);

    board
        .position_board
        .get_mut(x_end)
        .unwrap()
        .insert(y_end, start_piece);

    let mut found_piece = board
        .find_piece(x_start, y_start, &last_move_color)
        .unwrap();

    found_piece.set_pos(x_end, y_end);
    match found_piece {
        AnyPiece::King(king) => match last_move_color {
            PieceColor::WHITE => board.white_king_position = (x_end, y_end),
            PieceColor::BLACK => board.black_king_position = (x_end, y_end),
        },
        _ => (),
    };

    let opposing_king_pos = match king_color_being_checked {
        PieceColor::WHITE => board.white_king_position,
        PieceColor::BLACK => board.black_king_position,
    };

    let found_check = board.can_any_piece_check_king(
        opposing_king_pos.0,
        opposing_king_pos.1,
        king_color_being_checked,
    );

    // Clean up
    let start_piece = board.position_board.get_mut(x_end).unwrap().remove(y_end);
    board
        .position_board
        .get_mut(x_end)
        .unwrap()
        .insert(y_end, end_piece);
    board
        .position_board
        .get_mut(x_start)
        .unwrap()
        .remove(y_start);
    board
        .position_board
        .get_mut(x_start)
        .unwrap()
        .insert(y_start, start_piece);

    let mut found_piece = board.find_piece(x_end, y_end, &last_move_color).unwrap();
    found_piece.set_pos(x_start, y_start);
    match found_piece {
        AnyPiece::King(king) => match last_move_color {
            PieceColor::WHITE => board.white_king_position = king.get_pos(),
            PieceColor::BLACK => board.black_king_position = king.get_pos(),
        },
        _ => (),
    };
    found_check
}

fn does_piece_check(
    piece: AnyPiece,
    opposing_king_x: usize,
    opposing_king_y: usize,
    quick_board: &Vec<Vec<QuickPiece>>,
) -> bool {
    piece.can_move(opposing_king_x, opposing_king_y, &quick_board)
}
