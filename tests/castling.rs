use chess::board::Board;
use chess::piece_types::PieceColor;
use chess::piece_types::QuickPiece;
use chess::pieces::bishop::Bishop;
use chess::pieces::king::King;
use chess::pieces::rook::Rook;
use chess::pieces::AnyPiece;
use std::collections::HashMap;

mod common;

#[test]
fn test_can_castle_white_only_no_moves() {
    let mut pos_board = Vec::new();
    for index in 0..8 {
        pos_board.push(vec![]);
        for _ in 0..8 {
            pos_board.get_mut(index).unwrap().push(QuickPiece::EMPTY);
        }
    }

    assert_eq!(pos_board.len(), 8);
    for x in &pos_board {
        assert_eq!(x.len(), 8);
    }

    let mut board = Board {
        position_board: pos_board,
        live_white_pieces: vec![],
        live_black_pieces: vec![],
        white_king_position: (0, 0),
        black_king_position: (0, 0),
        last_move_color: PieceColor::WHITE,
        played_moves: Vec::new(),
        board_state_hashes: HashMap::new(),
    };

    common::insert_piece_into_board(
        AnyPiece::King(King::new(4, 0, PieceColor::WHITE)),
        &PieceColor::WHITE,
        4,
        0,
        &mut board,
    );
    common::insert_piece_into_board(
        AnyPiece::Rook(Rook::new(0, 0, PieceColor::WHITE)),
        &PieceColor::WHITE,
        0,
        0,
        &mut board,
    );
    common::insert_piece_into_board(
        AnyPiece::Rook(Rook::new(7, 0, PieceColor::WHITE)),
        &PieceColor::WHITE,
        7,
        0,
        &mut board,
    );

    assert_eq!(
        board.can_castle_king(&PieceColor::WHITE, 6),
        true,
        "The white King should be able to castle."
    );
    assert_eq!(
        board.can_castle_king(&PieceColor::WHITE, 2),
        true,
        "The white King should be able to castle."
    );

    common::insert_piece_into_board(
        AnyPiece::Rook(Rook::new(5, 2, PieceColor::BLACK)),
        &PieceColor::BLACK,
        5,
        2,
        &mut board,
    );

    assert_eq!(
        board.can_castle_king(&PieceColor::WHITE, 6),
        false,
        "The white King should not be able to castle with rook on 5,2"
    );
    assert_eq!(
        board.can_castle_king(&PieceColor::WHITE, 2),
        true,
        "The white King should be able to castle with rook on 5,2"
    );
    common::insert_piece_into_board(
        AnyPiece::Bishop(Bishop::new(4, 1, PieceColor::BLACK)),
        &PieceColor::BLACK,
        5,
        2,
        &mut board,
    );
}

#[test]
fn test_can_castle_black_only_no_moves() {
    let mut pos_board = Vec::new();
    for index in 0..8 {
        pos_board.push(vec![]);
        for _ in 0..8 {
            pos_board.get_mut(index).unwrap().push(QuickPiece::EMPTY);
        }
    }

    assert_eq!(pos_board.len(), 8);
    for x in &pos_board {
        assert_eq!(x.len(), 8);
    }

    let mut board = Board {
        position_board: pos_board,
        live_white_pieces: vec![],
        live_black_pieces: vec![],
        white_king_position: (0, 0),
        black_king_position: (0, 0),
        last_move_color: PieceColor::WHITE,
        played_moves: Vec::new(),
        board_state_hashes: HashMap::new(),
    };

    common::insert_piece_into_board(
        AnyPiece::King(King::new(4, 7, PieceColor::BLACK)),
        &PieceColor::BLACK,
        4,
        7,
        &mut board,
    );
    common::insert_piece_into_board(
        AnyPiece::Rook(Rook::new(0, 7, PieceColor::BLACK)),
        &PieceColor::BLACK,
        0,
        7,
        &mut board,
    );
    common::insert_piece_into_board(
        AnyPiece::Rook(Rook::new(7, 7, PieceColor::BLACK)),
        &PieceColor::BLACK,
        7,
        7,
        &mut board,
    );

    assert_eq!(
        board.can_castle_king(&PieceColor::BLACK, 6),
        true,
        "The white King should be able to castle."
    );
    assert_eq!(
        board.can_castle_king(&PieceColor::BLACK, 2),
        true,
        "The white King should be able to castle."
    );

    common::insert_piece_into_board(
        AnyPiece::Rook(Rook::new(5, 2, PieceColor::WHITE)),
        &PieceColor::WHITE,
        5,
        2,
        &mut board,
    );

    assert_eq!(
        board.can_castle_king(&PieceColor::BLACK, 6),
        false,
        "The white King should not be able to castle with rook on 5,2"
    );
    assert_eq!(
        board.can_castle_king(&PieceColor::BLACK, 2),
        true,
        "The white King should be able to castle with rook on 5,2"
    );
    common::insert_piece_into_board(
        AnyPiece::Bishop(Bishop::new(4, 6, PieceColor::WHITE)),
        &PieceColor::WHITE,
        4,
        6,
        &mut board,
    );
}
