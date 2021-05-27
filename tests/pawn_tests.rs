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
        pawn.can_move(4, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 3, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        3,
        4,
        4
    );
}

#[test]
fn test_can_move_black_forward() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 4, PieceColor::BLACK);

    assert_eq!(
        pawn.can_move(4, 5, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        3,
        4,
        4
    );
}

#[test]
fn test_can_move_forward_white_right() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
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
        pawn.can_move(5, 3, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        5,
        3,
        4,
        4
    );

    common::insert_quick_piece_into_board(PIECE(PieceColor::BLACK), 5, 5, &mut board);
    assert_eq!(
        pawn.can_move(5, 5, &board),
        true,
        "Expected to be able to take to {}, {} from {},{}",
        5,
        5,
        4,
        4
    );
}

#[test]
fn test_can_move_forward_black_right() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 4, PieceColor::BLACK);

    assert_eq!(
        pawn.can_move(3, 5, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        3,
        5,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(5, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        3,
        4,
        4
    );
    common::insert_quick_piece_into_board(PIECE(PieceColor::WHITE), 5, 3, &mut board);
    assert_eq!(
        pawn.can_move(5, 3, &board),
        true,
        "Expected to be able to take to {}, {} from {},{}",
        5,
        3,
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
        6,
        8,
        7,
        7
    );
}

#[test]
fn test_can_move_forward_white_left() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(3, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        5,
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

    common::insert_quick_piece_into_board(PIECE(PieceColor::BLACK), 3, 5, &mut board);
    assert_eq!(
        pawn.can_move(3, 5, &board),
        true,
        "Expected to be able to take to {}, {} from {},{}",
        3,
        5,
        4,
        4
    );
}

#[test]
fn test_can_move_forward_black_left() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 4, PieceColor::BLACK);

    assert_eq!(
        pawn.can_move(3, 5, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        3,
        5,
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
    common::insert_quick_piece_into_board(PIECE(PieceColor::WHITE), 3, 3, &mut board);
    assert_eq!(
        pawn.can_move(3, 3, &board),
        true,
        "Expected to be able to take to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
}

#[test]
fn test_move_two_initial_black() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 4, PieceColor::BLACK);

    assert_eq!(
        pawn.can_move(4, 5, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 6, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        6,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 7, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        7,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        3,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        2,
        4,
        4
    );
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 3, &mut board);
    assert_eq!(
        pawn.can_move(4, 2, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{} through black piece at {},{}",
        4,
        2,
        4,
        4,
        4,
        3
    );
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 3, &mut board);
    assert_eq!(
        pawn.can_move(4, 2, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{} through white piece at {},{}",
        4,
        2,
        4,
        4,
        4,
        3
    );
    assert_eq!(
        pawn.can_move(4, 1, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        1,
        4,
        4
    );
}

#[test]
fn test_move_two_initial_white() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        pawn.can_move(4, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        6,
        4,
        4
    );
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 5, &mut board);
    assert_eq!(
        pawn.can_move(4, 6, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{} through black piece at {},{}",
        4,
        6,
        4,
        4,
        4,
        5
    );
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 5, &mut board);
    assert_eq!(
        pawn.can_move(4, 6, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{} through white piece at {},{}",
        4,
        6,
        4,
        4,
        4,
        5
    );

    assert_eq!(
        pawn.can_move(4, 7, &board),
        false,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        5,
        4,
        4
    );

    assert_eq!(
        pawn.can_move(4, 3, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        3,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 2, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        2,
        4,
        4
    );
    assert_eq!(
        pawn.can_move(4, 1, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        1,
        4,
        4
    );
}

#[test]
fn test_moves_on_board_white_start() {
    let mut board = common::create_board_with_piece(4, 1, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(4, 1, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 1, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = vec![(3, 2), (4, 2), (5, 2), (4, 3)];

    expected_possible_moves.sort();
    let mut actual_possible_moves = pawn.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}
#[test]
fn test_moves_on_board_black_start() {
    let mut board = common::create_board_with_piece(4, 6, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 6, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 6, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = vec![(3, 5), (4, 5), (5, 5), (4, 4)];

    expected_possible_moves.sort();
    let mut actual_possible_moves = pawn.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}

#[test]
fn test_moves_on_board_black_has_moved() {
    let mut board = common::create_board_with_piece(4, 5, QuickPiece::PIECE(PieceColor::BLACK));
    let pawn = Pawn::new(4, 5, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 5, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = vec![(3, 4), (4, 4), (5, 4)];

    expected_possible_moves.sort();
    let mut actual_possible_moves = pawn.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}

#[test]
fn test_moves_on_board_white_has_moved() {
    let mut board = common::create_board_with_piece(4, 2, QuickPiece::PIECE(PieceColor::WHITE));
    let pawn = Pawn::new(4, 2, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 2, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = vec![(3, 3), (4, 3), (5, 3)];

    expected_possible_moves.sort();
    let mut actual_possible_moves = pawn.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}
