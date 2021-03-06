mod common;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::bishop::Bishop;
use chess::pieces::PieceMove;

#[test]
fn test_can_move_no_where() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let bishop = Bishop::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        bishop.can_move(4, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {}, {}",
        4,
        4,
        4,
        4
    );
}

#[test]
fn test_can_move_lower_right() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let bishop = Bishop::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        bishop.can_move(3, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        5,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(2, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        2,
        6,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(1, 7, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        1,
        7,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(0, 8, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        0,
        8,
        4,
        4
    );
}
#[test]
fn test_can_move_upper_right() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let bishop = Bishop::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        bishop.can_move(5, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        5,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(6, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        6,
        6,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(7, 7, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        7,
        7,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(8, 8, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        8,
        8,
        4,
        4
    );
}

#[test]
fn test_can_move_upper_left() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let bishop = Bishop::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        bishop.can_move(5, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(6, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        6,
        2,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(7, 1, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        7,
        1,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(8, 0, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        8,
        0,
        4,
        4
    );
}

#[test]
fn test_can_move_lower_left() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let bishop = Bishop::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        bishop.can_move(3, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(2, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(1, 1, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        1,
        1,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(0, 0, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        0,
        0,
        4,
        4
    );
}

#[test]
fn test_can_move_through_piece() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let bishop = Bishop::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 5, &mut board);

    assert_eq!(
        bishop.can_move(6, 6, &board),
        false,
        "Expected to not be able to move through some color to {}, {} from {},{}",
        6,
        6,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(2, 2, &board),
        false,
        "Expected to not be able to move through some color to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(6, 2, &board),
        false,
        "Expected to not be able to move through some color to {}, {} from {},{}",
        6,
        2,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(2, 6, &board),
        false,
        "Expected to not be able to move through some color to {}, {} from {},{}",
        2,
        6,
        4,
        4
    );

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 5, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 3, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 5, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 3, 5, &mut board);

    assert_eq!(
        bishop.can_move(6, 6, &board),
        false,
        "Expected to not be able to move through opposing color to {}, {} from {},{}",
        6,
        6,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(2, 2, &board),
        false,
        "Expected to not be able to move through opposing color to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(6, 2, &board),
        false,
        "Expected to not be able to move through opposing color to {}, {} from {},{}",
        6,
        2,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(2, 6, &board),
        false,
        "Expected to not be able to move through opposing color to {}, {} from {},{}",
        2,
        6,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_same_color() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let bishop = Bishop::new(4, 4, PieceColor::WHITE);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 5, &mut board);

    // @TODO This should be replaced with an actual insertion function.  probably when I add the actual function impls to board
    assert_eq!(
        bishop.can_move(5, 5, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        5,
        5,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(3, 3, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(5, 3, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(3, 5, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        3,
        5,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_opposing_color() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let bishop = Bishop::new(4, 4, PieceColor::BLACK);
    // @TODO This should be replaced with an actual insertion function.  probably when I add the actual function impls to board
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 5, &mut board);
    assert_eq!(
        bishop.can_move(5, 5, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        5,
        5,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(3, 3, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(5, 3, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        bishop.can_move(3, 5, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        3,
        5,
        4,
        4
    );
}

#[test]
fn test_moves_on_board_4_4() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let bishop = Bishop::new(4, 4, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 4, &mut board);

    let mut expected_possible_moves = vec![
        (3, 3),
        (2, 2),
        (1, 1),
        (0, 0),
        (5, 5),
        (6, 6),
        (7, 7),
        (3, 5),
        (2, 6),
        (1, 7),
        (5, 3),
        (6, 2),
        (7, 1),
    ];
    expected_possible_moves.sort();
    let mut actual_possible_moves = bishop.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}

#[test]
fn test_moves_on_board_0_0() {
    let mut board = common::create_board_with_piece(0, 0, QuickPiece::PIECE(PieceColor::BLACK));
    let bishop = Bishop::new(0, 0, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 4, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = (1..8).zip(1..8).collect();
    expected_possible_moves.sort();
    let mut actual_possible_moves = bishop.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}

#[test]
fn test_moves_on_board_0_7() {
    let mut board = common::create_board_with_piece(0, 7, QuickPiece::PIECE(PieceColor::BLACK));
    let bishop = Bishop::new(0, 7, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 4, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = (1..8).zip((0..7).rev()).collect();
    expected_possible_moves.sort();
    let mut actual_possible_moves = bishop.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}

#[test]
fn test_moves_on_board_7_0() {
    let mut board = common::create_board_with_piece(7, 0, QuickPiece::PIECE(PieceColor::BLACK));
    let bishop = Bishop::new(7, 0, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 4, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = (0..7).rev().zip(1..8).collect();
    expected_possible_moves.sort();
    let mut actual_possible_moves = bishop.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}

#[test]
fn test_moves_on_board_7_7() {
    let mut board = common::create_board_with_piece(7, 7, QuickPiece::PIECE(PieceColor::BLACK));
    let bishop = Bishop::new(7, 7, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 4, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = (0..7).rev().zip((0..7).rev()).collect();
    expected_possible_moves.sort();
    let mut actual_possible_moves = bishop.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}
