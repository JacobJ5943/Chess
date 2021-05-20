mod common;
use chess::piece_types::QuickPiece::PIECE;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::queen::Queen;
use chess::pieces::PieceMove;

#[test]
fn test_can_move_no_where() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(4, 4, &board),
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
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(3, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        5,
        4,
        4
    );
    assert_eq!(
        queen.can_move(2, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        2,
        6,
        4,
        4
    );
    assert_eq!(
        queen.can_move(1, 7, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        1,
        7,
        4,
        4
    );
    assert_eq!(
        queen.can_move(0, 8, &board),
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
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(5, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        5,
        4,
        4
    );
    assert_eq!(
        queen.can_move(6, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        6,
        6,
        4,
        4
    );
    assert_eq!(
        queen.can_move(7, 7, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        7,
        7,
        4,
        4
    );
    assert_eq!(
        queen.can_move(8, 8, &board),
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
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(5, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        queen.can_move(6, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        6,
        2,
        4,
        4
    );
    assert_eq!(
        queen.can_move(7, 1, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        7,
        1,
        4,
        4
    );
    assert_eq!(
        queen.can_move(8, 0, &board),
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
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(3, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        queen.can_move(2, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
    assert_eq!(
        queen.can_move(1, 1, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        1,
        1,
        4,
        4
    );
    assert_eq!(
        queen.can_move(0, 0, &board),
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
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 4, &mut board);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 3, &mut board);

    assert_eq!(
        queen.can_move(4, 6, &board),
        false,
        "Expected to not be able to move through same color piece at {},{}, to space {},{} from {},{}",4,5,4,6,4,4
    );
    assert_eq!(
        queen.can_move(2, 4, &board),
        false,

        "Expected to not be able to move through same color piece at {},{}, to space {},{} from {},{}",3,4,2,4,4,4
    );
    assert_eq!(
        queen.can_move(6, 4, &board),
        false,

        "Expected to not be able to move through same color piece at {},{}, to space {},{} from {},{}",5,4,6,4,4,4

    );
    assert_eq!(
        queen.can_move(4, 2, &board),
        false,

        "Expected to not be able to move through same color piece at {},{}, to space {},{} from {},{}",4,3,4,2,4,4

    );

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 3, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 5, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 3, &mut board);

    assert_eq!(
        queen.can_move(4, 6, &board),
        false,
        "Expected to not be able to move through opposing color piece at {},{}, to space {},{} from {},{}",4,5,4,6,4,4

    );
    assert_eq!(
        queen.can_move(2, 4, &board),
        false,

        "Expected to not be able to move through opposing color piece at {},{}, to space {},{} from {},{}",3,4,4,6,4,4

    );
    assert_eq!(
        queen.can_move(6, 4, &board),
        false,

        "Expected to not be able to move through opposing color piece at {},{}, to space {},{} from {},{}",5,4,6,4,4,4

    );
    assert_eq!(
        queen.can_move(4, 2, &board),
        false,

        "Expected to not be able to move through opposing color piece at {},{}, to space {},{} from {},{}",4,3,4,2,4,4

    );
}

#[test]
fn test_can_move_through_piece_diag() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 5, &mut board);

    assert_eq!(
        queen.can_move(6, 6, &board),
        false,
        "Expected to not be able to move through some color to {}, {} from {},{}",
        6,
        6,
        4,
        4
    );
    assert_eq!(
        queen.can_move(2, 2, &board),
        false,
        "Expected to not be able to move through some color to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
    assert_eq!(
        queen.can_move(6, 2, &board),
        false,
        "Expected to not be able to move through some color to {}, {} from {},{}",
        6,
        2,
        4,
        4
    );
    assert_eq!(
        queen.can_move(2, 6, &board),
        false,
        "Expected to not be able to move through some color to {}, {} from {},{}",
        2,
        6,
        4,
        4
    );

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 5, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 3, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 5, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 3, 5, &mut board);

    assert_eq!(
        queen.can_move(6, 6, &board),
        false,
        "Expected to not be able to move through opposing color to {}, {} from {},{}",
        6,
        6,
        4,
        4
    );
    assert_eq!(
        queen.can_move(2, 2, &board),
        false,
        "Expected to not be able to move through opposing color to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
    assert_eq!(
        queen.can_move(6, 2, &board),
        false,
        "Expected to not be able to move through opposing color to {}, {} from {},{}",
        6,
        2,
        4,
        4
    );
    assert_eq!(
        queen.can_move(2, 6, &board),
        false,
        "Expected to not be able to move through opposing color to {}, {} from {},{}",
        2,
        6,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_same_color_diag() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 5, &mut board);

    assert_eq!(
        queen.can_move(5, 5, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        5,
        5,
        4,
        4
    );
    assert_eq!(
        queen.can_move(3, 3, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        queen.can_move(5, 3, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        queen.can_move(3, 5, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        3,
        5,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_same_color_cross() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 3, &mut board);

    assert_eq!(
        queen.can_move(4, 5, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        queen.can_move(3, 4, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        3,
        4,
        4,
        4
    );
    assert_eq!(
        queen.can_move(5, 4, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        queen.can_move(4, 3, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        4,
        3,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_opposing_color_cross() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 3, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 5, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 3, &mut board);

    assert_eq!(
        queen.can_move(4, 5, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        queen.can_move(3, 4, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        3,
        4,
        4,
        4
    );
    assert_eq!(
        queen.can_move(5, 4, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        queen.can_move(4, 3, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        4,
        3,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_opposing_color_diag() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let queen = Queen::new(4, 4, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 5, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 3, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 3, 5, &mut board);

    assert_eq!(
        queen.can_move(5, 5, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        5,
        5,
        4,
        4
    );
    assert_eq!(
        queen.can_move(3, 3, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        queen.can_move(5, 3, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        queen.can_move(3, 5, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        3,
        5,
        4,
        4
    );
}

#[test]
fn test_can_move_right() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(4, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        queen.can_move(4, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        6,
        4,
        4
    );
    assert_eq!(
        queen.can_move(4, 7, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        7,
        4,
        4
    );
    assert_eq!(
        queen.can_move(4, 8, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        0,
        8,
        4,
        4
    );
}
#[test]
fn test_can_move_down() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(5, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        queen.can_move(6, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        6,
        4,
        4,
        4
    );
    assert_eq!(
        queen.can_move(7, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        7,
        4,
        4,
        4
    );
    assert_eq!(
        queen.can_move(8, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        8,
        4,
        4,
        4
    );
}

#[test]
fn test_can_move_left() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(4, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        3,
        4,
        4
    );
    assert_eq!(
        queen.can_move(4, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        2,
        4,
        4
    );
    assert_eq!(
        queen.can_move(4, 1, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        1,
        4,
        4
    );
    assert_eq!(
        queen.can_move(4, 0, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        0,
        4,
        4
    );
}

#[test]
fn test_can_move_up() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let queen = Queen::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        queen.can_move(3, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        queen.can_move(2, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
    assert_eq!(
        queen.can_move(1, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        1,
        1,
        4,
        4
    );
    assert_eq!(
        queen.can_move(0, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        0,
        4,
        4,
        4
    );
}
#[test]
fn test_moves_on_board_4_4() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let queen = Queen::new(4, 4, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 4, 4, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = vec![
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
    expected_possible_moves.append(&mut vec![
        (0, 4),
        (1, 4),
        (2, 4),
        (3, 4),
        (5, 4),
        (6, 4),
        (7, 4),
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 3),
        (4, 5),
        (4, 6),
        (4, 7),
    ]);
    expected_possible_moves.sort();
    let mut actual_possible_moves = queen.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}
#[test]
fn test_moves_on_board_0_0() {
    let mut board = common::create_board_with_piece(0, 0, QuickPiece::PIECE(PieceColor::BLACK));
    let queen = Queen::new(0, 0, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 0, 0, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = (1..8).zip(1..8).collect();
    expected_possible_moves.append(&mut vec![
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (0, 7),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
    ]);
    expected_possible_moves.sort();
    let mut actual_possible_moves = queen.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}

#[test]
fn test_moves_on_board_7_7() {
    let mut board = common::create_board_with_piece(7, 7, QuickPiece::PIECE(PieceColor::BLACK));
    let queen = Queen::new(7, 7, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 7, 7, &mut board);

    let mut expected_possible_moves: Vec<(usize, usize)> = (0..7).rev().zip((0..7).rev()).collect();
    expected_possible_moves.append(&mut vec![
        (0, 7),
        (1, 7),
        (2, 7),
        (3, 7),
        (4, 7),
        (5, 7),
        (6, 7),
        (7, 0),
        (7, 1),
        (7, 2),
        (7, 3),
        (7, 4),
        (7, 5),
        (7, 6),
    ]);
    expected_possible_moves.sort();
    let mut actual_possible_moves = queen.moves_on_board();
    actual_possible_moves.sort();

    assert_eq!(actual_possible_moves, expected_possible_moves);
}
