mod common;
use chess::piece_types::QuickPiece::PIECE;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::king::King;
use chess::pieces::PieceMove;

#[test]
fn test_can_move_no_where() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let king = King::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        king.can_move(4, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {}, {}",
        4,
        4,
        4,
        4
    );
}

#[test]
fn test_can_move_right() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let king = King::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        king.can_move(4, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        king.can_move(4, 6, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        6,
        4,
        4
    );
}
#[test]
fn test_can_move_down() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let king = King::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        king.can_move(5, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        king.can_move(6, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        6,
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
    let king = King::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        king.can_move(4, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        3,
        4,
        4
    );
    assert_eq!(
        king.can_move(4, 2, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        4,
        2,
        4,
        4
    );
}

#[test]
fn test_can_move_up() {
    // First create the piece and the board
    //assert_eq!(false, true);
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let king = King::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        king.can_move(3, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        king.can_move(2, 4, &board),
        false,
        "Expected to not be able to move to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_same_color() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let king = King::new(4, 4, PieceColor::WHITE);
    // @TODO This should be replaced with an actual insertion function.  probably when I add the actual function impls to board
    board.get_mut(4).unwrap().remove(5);
    board
        .get_mut(4)
        .unwrap()
        .insert(5, QuickPiece::PIECE(PieceColor::WHITE));

    board.get_mut(3).unwrap().remove(4);
    board
        .get_mut(3)
        .unwrap()
        .insert(4, QuickPiece::PIECE(PieceColor::WHITE));

    board.get_mut(5).unwrap().remove(4);
    board
        .get_mut(5)
        .unwrap()
        .insert(4, QuickPiece::PIECE(PieceColor::WHITE));

    board.get_mut(4).unwrap().remove(3);
    board
        .get_mut(4)
        .unwrap()
        .insert(3, QuickPiece::PIECE(PieceColor::WHITE));

    assert_eq!(
        king.can_move(4, 5, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        king.can_move(3, 4, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        3,
        4,
        4,
        4
    );
    assert_eq!(
        king.can_move(5, 4, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        king.can_move(4, 3, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        4,
        3,
        4,
        4
    );
}

#[test]
fn test_can_move_capture_opposing_color() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::BLACK));
    let king = King::new(4, 4, PieceColor::BLACK);
    // @TODO This should be replaced with an actual insertion function.  probably when I add the actual function impls to board
    board.get_mut(4).unwrap().remove(5);
    board
        .get_mut(4)
        .unwrap()
        .insert(5, QuickPiece::PIECE(PieceColor::WHITE));

    board.get_mut(3).unwrap().remove(4);
    board
        .get_mut(3)
        .unwrap()
        .insert(4, QuickPiece::PIECE(PieceColor::WHITE));

    board.get_mut(5).unwrap().remove(4);
    board
        .get_mut(5)
        .unwrap()
        .insert(4, QuickPiece::PIECE(PieceColor::WHITE));

    board.get_mut(4).unwrap().remove(3);
    board
        .get_mut(4)
        .unwrap()
        .insert(3, QuickPiece::PIECE(PieceColor::WHITE));

    assert_eq!(
        king.can_move(4, 5, &board),
        true,
        "Expected to not capture opposing color piece at {},{} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        king.can_move(3, 4, &board),
        true,
        "Expected to not capture opposing color piece at {},{} from {},{}",
        3,
        4,
        4,
        4
    );
    assert_eq!(
        king.can_move(5, 4, &board),
        true,
        "Expected to not capture opposing color piece at {},{} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        king.can_move(4, 3, &board),
        true,
        "Expected to not capture opposing color piece at {},{} from {},{}",
        4,
        3,
        4,
        4
    );
}

#[test]
fn test_can_move_out_of_bounds_right() {
    let mut board = common::create_board_with_piece(7, 7, QuickPiece::PIECE(PieceColor::BLACK));
    let king = King::new(7, 7, PieceColor::BLACK);

    assert_eq!(
        false,
        king.can_move(7, 8, &board),
        "Expected to not capture same color piece at {},{} from {},{}",
        7,
        8,
        4,
        4
    );
}

#[test]
fn test_can_move_out_of_bounds_up() {
    let mut board = common::create_board_with_piece(7, 7, QuickPiece::PIECE(PieceColor::BLACK));
    let king = King::new(7, 7, PieceColor::BLACK);

    assert_eq!(
        false,
        king.can_move(8, 7, &board),
        "Expected to not capture same color piece at {},{} from {},{}",
        8,
        7,
        4,
        4
    );
}
