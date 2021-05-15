use chess::pieces::bishop::Bishop;
mod common;
use chess::board;
use chess::game;
use chess::piece_types::QuickPiece::PIECE;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::{AnyPiece, PieceMove};
use chess::pieces::king::King;
use chess::pieces::knight::Knight;

#[test]
fn test_check_same_color() {
    let pos_board = common::create_board_with_piece(4, 4, QuickPiece::PIECE(PieceColor::WHITE));
    let mut board = board::Board {
        position_board: pos_board,
        live_white_pieces: vec![],
        live_black_pieces: vec![],
        white_king_position: (0, 0),
        black_king_position: (0, 0),
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
        game::is_board_in_check(PieceColor::WHITE, &board),
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

    assert_eq!(game::is_board_in_check(PieceColor::WHITE, &board), true, "Expected false with white bishop at {},{} and black king at {},{}, and white king at {},{}", 4,4,6,6,6,3);
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

    assert_eq!(game::is_board_in_check(PieceColor::WHITE, &board), false, "Expected not to be able to check from {},{} through knight at {},{} to opposing king at {},{}",4,4,5,5,6,6);
}
