mod common;
use chess::piece_types::QuickPiece::PIECE;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::rook::Rook;
use chess::pieces::PieceMove;

#[test]
fn test_can_move_no_where() {
    let board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let rook = Rook::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        rook.can_move(4, 4, &board),
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
    let rook = Rook::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        rook.can_move(4, 5, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        rook.can_move(4, 6, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        6,
        4,
        4
    );
    assert_eq!(
        rook.can_move(4, 7, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        7,
        4,
        4
    );
    assert_eq!(
        rook.can_move(4, 8, &board),
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
    let rook = Rook::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        rook.can_move(5, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        rook.can_move(6, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        6,
        4,
        4,
        4
    );
    assert_eq!(
        rook.can_move(7, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        7,
        4,
        4,
        4
    );
    assert_eq!(
        rook.can_move(8, 4, &board),
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
    let rook = Rook::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        rook.can_move(4, 3, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        3,
        4,
        4
    );
    assert_eq!(
        rook.can_move(4, 2, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        2,
        4,
        4
    );
    assert_eq!(
        rook.can_move(4, 1, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        4,
        1,
        4,
        4
    );
    assert_eq!(
        rook.can_move(4, 0, &board),
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
    let rook = Rook::new(4, 4, PieceColor::WHITE);

    assert_eq!(
        rook.can_move(3, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        3,
        3,
        4,
        4
    );
    assert_eq!(
        rook.can_move(2, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        2,
        2,
        4,
        4
    );
    assert_eq!(
        rook.can_move(1, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        1,
        1,
        4,
        4
    );
    assert_eq!(
        rook.can_move(0, 4, &board),
        true,
        "Expected to be able to move to {}, {} from {},{}",
        0,
        4,
        4,
        4
    );
}

#[test]
fn test_can_move_through_piece() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let rook = Rook::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),4,5,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),3,4,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),5,4,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),4,3,&mut board);

    assert_eq!(
        rook.can_move(4, 6, &board),
        false,
        "Expected to not be able to move through same color piece at {},{}, to space {},{} from {},{}",4,5,4,6,4,4
    );
    assert_eq!(
        rook.can_move(2, 4, &board),
        false,

        "Expected to not be able to move through same color piece at {},{}, to space {},{} from {},{}",3,4,2,4,4,4
   );
    assert_eq!(
        rook.can_move(6, 4, &board),
        false,

        "Expected to not be able to move through same color piece at {},{}, to space {},{} from {},{}",5,4,6,4,4,4

    );
    assert_eq!(
        rook.can_move(4, 2, &board),
        false,

        "Expected to not be able to move through same color piece at {},{}, to space {},{} from {},{}",4,3,4,2,4,4

    );

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK),4,5,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK),3,4,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK),5,4,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::BLACK),4,3,&mut board);

    assert_eq!(
        rook.can_move(4, 6, &board),
        false,
        "Expected to not be able to move through opposing color piece at {},{}, to space {},{} from {},{}",4,5,4,6,4,4

    );
    assert_eq!(
        rook.can_move(2, 4, &board),
        false,

        "Expected to not be able to move through opposing color piece at {},{}, to space {},{} from {},{}",3,4,4,6,4,4

    );
    assert_eq!(
        rook.can_move(6, 4, &board),
        false,

        "Expected to not be able to move through opposing color piece at {},{}, to space {},{} from {},{}",5,4,6,4,4,4

    );
    assert_eq!(
        rook.can_move(4, 2, &board),
        false,

        "Expected to not be able to move through opposing color piece at {},{}, to space {},{} from {},{}",4,3,4,2,4,4

    );
}

#[test]
fn test_can_move_capture_same_color() {
    let mut board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let rook = Rook::new(4, 4, PieceColor::WHITE);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),4,5,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),3,4,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),5,4,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),4,3,&mut board);

    assert_eq!(
        rook.can_move(4, 5, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        rook.can_move(3, 4, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        3,
        4,
        4,
        4
    );
    assert_eq!(
        rook.can_move(5, 4, &board),
        false,
        "Expected to not capture same color piece at {},{} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        rook.can_move(4, 3, &board),
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
    let rook = Rook::new(4, 4, PieceColor::BLACK);

    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),4,5,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),3,4,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),5,4,&mut board);
    common::insert_quick_piece_into_board(QuickPiece::PIECE(PieceColor::WHITE),4,3,&mut board);

    assert_eq!(
        rook.can_move(4, 5, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        4,
        5,
        4,
        4
    );
    assert_eq!(
        rook.can_move(3, 4, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        3,
        4,
        4,
        4
    );
    assert_eq!(
        rook.can_move(5, 4, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        5,
        4,
        4,
        4
    );
    assert_eq!(
        rook.can_move(4, 3, &board),
        true,
        "Expected to capture opposing color piece at {},{} from {},{}",
        4,
        3,
        4,
        4
    );
}
