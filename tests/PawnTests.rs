mod common;
use chess::piece_types::QuickPiece::PIECE;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::pawn::Pawn;
use chess::pieces::PieceMove;

#[test]
fn test_can_move_no_where() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(4, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {}, {}",
        4,
        4,
        4,
        4
    );
}

#[test]
fn test_can_move_white_forward() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(5, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(3, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        3,
        4,
        4,
        4
    );
}

#[test]
fn test_can_move_black_forward() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 4, PieceColor::BLACK);

    assert_eq!(
        pawn.can_move(5, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(3, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        4,
        4,
        4
    );
}

#[test]
fn test_can_move_forward_white_right() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(5, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        5,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(3, 5, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        3,
        5,
        4,
        4
    );
}

#[test]
fn test_can_move_forward_black_right() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 4, PieceColor::BLACK);

    assert_eq!(
        pawn.can_move(5, 5, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        5,
        5,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(3, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        5,
        4,
        4
    );
}

#[test]
fn test_can_move_out_of_bounds_white() {
    let board = common::create_board_with_piece(0, 7, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(0, 7, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(0, 8, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        0,
        8,
        0,
        7
    );
}


#[test]
fn test_can_move_out_of_bounds_black() {
    let board = common::create_board_with_piece(7, 7, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(7, 7, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(6, 8, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        0,
        7,
        6,
        8
    );
}

#[test]
fn test_can_move_forward_white_left() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(5, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(3, 3, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
}

#[test]
fn test_can_move_forward_black_left() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(5, 3, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(3, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
}
