mod common;
use chess::piece_types::QuickPiece::PIECE;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::knight::Knight;
use chess::pieces::PieceMove;

#[test]
fn test_can_move_no_where() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        knight.can_move(4, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {}, {}",
        4,
        4,
        4,
        4
    );
}

#[test]
fn test_can_move_upper_right() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        knight.can_move(2, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        2,
        5,
        4,
        4
    );
    assert_eq!(
        knight.can_move(2, 6, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        2,
        6,
        4,
        4
    );
    assert_eq!(
        knight.can_move(3, 5, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        3,
        5,
        4,
        4
    );
    assert_eq!(
        knight.can_move(3, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        6,
        4,
        4
    );
}

#[test]
fn test_can_move_lower_right() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        knight.can_move(5, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        6,
        4,
        4
    );
    assert_eq!(
        knight.can_move(6, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        6,
        5,
        4,
        4
    );
    assert_eq!(
        knight.can_move(6, 6, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        6,
        6,
        4,
        4
    );
    assert_eq!(
        knight.can_move(5, 5, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        5,
        5,
        4,
        4
    );
}

#[test]
fn test_can_move_lower_left() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        knight.can_move(5, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        2,
        4,
        4
    );
    assert_eq!(
        knight.can_move(6, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        6,
        3,
        4,
        4
    );
    assert_eq!(
        knight.can_move(5, 3, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        5,
        3,
        4,
        4
    );
    assert_eq!(
        knight.can_move(6, 2, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        6,
        2,
        4,
        4
    );
}

#[test]
fn test_can_move_upper_left() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        knight.can_move(3, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        2,
        4,
        4
    );
    assert_eq!(
        knight.can_move(2, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        2,
        3,
        4,
        4
    );
    assert_eq!(
        knight.can_move(3, 3, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        knight.can_move(2, 2, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
}

//@TODO Maybe expand these. I'm too tired as of writing this to have all 8 cases of when a knight can capture a piece.  If it passes one I can't imagine it wouldn't pass them all
#[test]
fn test_can_move_capture_same_color() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(4, 4, PieceColor::WHITE);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 5, 6, &mut board);
    assert_eq!(
        knight.can_move(5, 6, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        5,
        6,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_opposing_color() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE), 4, 4, &mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK), 5, 6, &mut board);

    assert_eq!(
        knight.can_move(5, 6, &board),
        true,
        "Expected to capture same opposing piece at {},{} from {},{}",
        5,
        6,
        4,
        4
    );
}

#[test]
fn test_can_move_out_of_bounds_upper_right() {
    let board = common::create_board_with_piece(7, 7, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(7, 7, PieceColor::WHITE);

    assert_eq!(
        knight.can_move(5, 8, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        5,
        8,
        7,
        7
    );
    assert_eq!(
        knight.can_move(6, 9, &board),
        false,
        "Expected not to not be able to move to {}, {} from {},{}",
        6,
        9,
        7,
        7
    );
}

#[test]
fn test_can_move_out_of_bounds_lower_right() {
    let board = common::create_board_with_piece(7, 7, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(7, 7, PieceColor::WHITE);

    assert_eq!(
        knight.can_move(8, 9, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        8,
        9,
        7,
        7
    );
    assert_eq!(
        knight.can_move(9, 8, &board),
        false,
        "Expected not to not be able to move to {}, {} from {},{}",
        9,
        8,
        7,
        7
    );
}

#[test]
fn test_can_move_out_of_bounds_lower_left() {
    let board = common::create_board_with_piece(7, 7, QuickPiece::PIECE(PieceColor::WHITE));
    let knight = Knight::new(7, 7, PieceColor::WHITE);

    assert_eq!(
        knight.can_move(9, 6, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        9,
        6,
        7,
        7
    );
    assert_eq!(
        knight.can_move(8, 5, &board),
        false,
        "Expected not to not be able to move to {}, {} from {},{}",
        8,
        5,
        7,
        7
    );
}
