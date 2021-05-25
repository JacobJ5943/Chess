use chess::pieces::bishop::Bishop;
mod common;
use chess::board;
use chess::game;
use chess::piece_types::QuickPiece::PIECE;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::king::King;
use chess::pieces::knight::Knight;
use chess::pieces::queen::Queen;
use chess::pieces::rook::Rook;
use chess::pieces::{AnyPiece, PieceMove};
use std::collections::BinaryHeap;

#[test]
fn test_check_same_color() {
    let pos_board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let mut board = board::Board {
        position_board: pos_board,
        live_white_pieces: vec![],
        live_black_pieces: vec![],
        white_king_position: (0, 0),
        black_king_position: (0, 0),
        last_move_color: PieceColor::BLACK,
    };

    let bishop = Bishop::new(4, 4, PieceColor::WHITE);
    board.live_white_pieces.push(AnyPiece::Bishop(bishop));

    board.position_board.get_mut(6).unwrap().remove(6);
    board
        .position_board
        .get_mut(6)
        .unwrap()
        .insert(6, QuickPiece::KING(PieceColor::WHITE));
    board
        .live_white_pieces
        .push(AnyPiece::King(King::new(6, 6, PieceColor::WHITE)));
    board.white_king_position = (6, 6);

    board.position_board.get_mut(6).unwrap().remove(3);
    board
        .position_board
        .get_mut(6)
        .unwrap()
        .insert(3, QuickPiece::KING(PieceColor::BLACK));
    board
        .live_black_pieces
        .push(AnyPiece::King(King::new(6, 3, PieceColor::BLACK)));
    board.black_king_position = (6, 3);

    assert_eq!(
        game::is_board_in_check(&PieceColor::WHITE, &board),
        false,
        "Expected false with white bishop at {},{} and white king at {},{} and black king at {},{}",
        4,
        4,
        6,
        6,
        6,
        3
    );

    board.position_board.get_mut(6).unwrap().remove(3);
    board
        .position_board
        .get_mut(6)
        .unwrap()
        .insert(3, QuickPiece::KING(PieceColor::WHITE));
    board
        .live_white_pieces
        .push(AnyPiece::King(King::new(6, 3, PieceColor::WHITE)));
    board.white_king_position = (6, 3);

    board.position_board.get_mut(6).unwrap().remove(6);
    board
        .position_board
        .get_mut(6)
        .unwrap()
        .insert(6, QuickPiece::KING(PieceColor::BLACK));
    board
        .live_black_pieces
        .push(AnyPiece::King(King::new(6, 6, PieceColor::BLACK)));
    board.black_king_position = (6, 6);

    assert_eq!(game::is_board_in_check(&PieceColor::WHITE, &board), true, "Expected false with white bishop at {},{} and black king at {},{}, and white king at {},{}", 4,4,6,6,6,3);
}

#[test]
fn test_check_through_pieces() {
    let pos_board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let mut board = board::Board {
        position_board: pos_board,
        live_white_pieces: vec![],
        live_black_pieces: vec![],
        white_king_position: (0, 0),
        black_king_position: (0, 0),
        last_move_color: PieceColor::BLACK,
    };

    let bishop = Bishop::new(4, 4, PieceColor::BLACK);
    board.live_white_pieces.push(AnyPiece::Bishop(bishop));
    let knight = Knight::new(5, 5, PieceColor::BLACK);
    board.live_white_pieces.push(AnyPiece::Knight(knight));

    board.position_board.get_mut(6).unwrap().remove(6);
    board
        .position_board
        .get_mut(6)
        .unwrap()
        .insert(6, QuickPiece::KING(PieceColor::WHITE));
    board
        .live_white_pieces
        .push(AnyPiece::King(King::new(6, 6, PieceColor::WHITE)));
    board.white_king_position = (6, 6);

    board.position_board.get_mut(6).unwrap().remove(3);
    board
        .position_board
        .get_mut(6)
        .unwrap()
        .insert(3, QuickPiece::KING(PieceColor::BLACK));
    board
        .live_black_pieces
        .push(AnyPiece::King(King::new(6, 3, PieceColor::BLACK)));
    board.black_king_position = (6, 3);

    assert_eq!(game::is_board_in_check(&PieceColor::WHITE, &board), false, "Expected not to be able to check from {},{} through knight at {},{} to opposing king at {},{}",4,4,5,5,6,6);
}

#[test]

fn will_move_be_in_check() {
    let pos_board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let mut board = board::Board {
        position_board: pos_board,
        live_white_pieces: vec![],
        live_black_pieces: vec![],
        white_king_position: (0, 0),
        black_king_position: (0, 0),
        last_move_color: PieceColor::BLACK,
    };

    common::insert_piece_into_board(
        AnyPiece::King(King::new(6, 6, PieceColor::WHITE)),
        &PieceColor::WHITE,
        6,
        6,
        &mut board,
    );
    common::insert_piece_into_board(
        AnyPiece::Bishop(Bishop::new(6, 2, PieceColor::BLACK)),
        &PieceColor::BLACK,
        6,
        2,
        &mut board,
    );

    assert_eq!(
        game::will_move_be_in_check(
            6,
            2,
            4,
            4,
            &PieceColor::BLACK,
            &PieceColor::WHITE,
            &mut board
        ),
        true,
        "Moving bishop from {},{} to {},{} should put king at check at {},{}",
        6,
        2,
        4,
        4,
        6,
        6
    );
    assert_eq!(
        game::will_move_be_in_check(
            6,
            2,
            3,
            3,
            &PieceColor::BLACK,
            &PieceColor::WHITE,
            &mut board
        ),
        false,
        "Moving bishop from {},{} to {},{} should not put king at check at {},{}",
        6,
        2,
        3,
        3,
        6,
        6
    );
}

#[test]
fn test_check_mate() {
    let pos_board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let mut board = board::Board {
        position_board: pos_board,
        live_white_pieces: vec![],
        live_black_pieces: vec![],
        white_king_position: (0, 0),
        black_king_position: (0, 0),
        last_move_color: PieceColor::BLACK,
    };

    common::insert_piece_into_board(
        AnyPiece::King(King::new(6, 6, PieceColor::WHITE)),
        &PieceColor::WHITE,
        6,
        6,
        &mut board,
    );

    common::insert_piece_into_board(
        AnyPiece::Bishop(Bishop::new(4, 4, PieceColor::BLACK)),
        &PieceColor::BLACK,
        4,
        4,
        &mut board,
    );

    assert_eq!(
        game::is_board_check_mate(&PieceColor::BLACK, &mut board),
        false,
        "With only 1 bishop The board should not be in check, but not mate."
    );

    common::insert_piece_into_board(
        AnyPiece::Queen(Queen::new(5, 5, PieceColor::BLACK)),
        &PieceColor::BLACK,
        5,
        5,
        &mut board,
    );

    assert_eq!(
        game::is_board_check_mate(&PieceColor::BLACK, &mut board),
        false,
        "With 1 bishop, and 1 queen The board should not be in check, but not mate."
    );

    common::insert_piece_into_board(
        AnyPiece::Rook(Rook::new(0, 7, PieceColor::BLACK)),
        &PieceColor::BLACK,
        0,
        7,
        &mut board,
    );

    assert_eq!(
        game::is_board_check_mate(&PieceColor::BLACK, &mut board),
        false,
        "With 1 bishop, and 1 queen, and 1 rook The board should not be in check, but not mate."
    );

    common::insert_piece_into_board(
        AnyPiece::Rook(Rook::new(7, 0, PieceColor::BLACK)),
        &PieceColor::BLACK,
        7,
        0,
        &mut board,
    );

    assert_eq!(
        game::is_board_check_mate(&PieceColor::BLACK, &mut board),
        true,
        "With 1 bishop, and 1 queen, and 2 rooks The board should be in checkmate."
    );
}
