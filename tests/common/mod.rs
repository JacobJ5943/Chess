use chess::board::Board;
use chess::piece_types;
use chess::piece_types::{PieceColor, QuickPiece};
use chess::pieces::AnyPiece;

/// Removes the QuickPiece that is at that given location and adds the piece to the appropriate living vector
#[allow(dead_code)]
pub fn insert_piece_into_board(
    piece: AnyPiece,
    piece_color: &PieceColor,
    x_coord: usize,
    y_coord: usize,
    board: &mut Board,
) {
    let owned_piece_color = match piece_color {
        PieceColor::WHITE => PieceColor::WHITE,
        PieceColor::BLACK => PieceColor::BLACK,
    };

    let quick_piece = match &piece {
        AnyPiece::King(_) => QuickPiece::KING(owned_piece_color),
        _ => QuickPiece::PIECE(owned_piece_color),
    };

    match &quick_piece {
        QuickPiece::KING(color) => match color {
            PieceColor::WHITE => board.white_king_position = (x_coord, y_coord),
            PieceColor::BLACK => board.black_king_position = (x_coord, y_coord),
        },
        _ => (),
    }

    match piece_color {
        PieceColor::WHITE => board.live_white_pieces.push(piece),
        PieceColor::BLACK => board.live_black_pieces.push(piece),
    }

    board
        .position_board
        .get_mut(x_coord)
        .unwrap()
        .remove(y_coord);
    board
        .position_board
        .get_mut(x_coord)
        .unwrap()
        .insert(y_coord, quick_piece);
}

// inserts a piece into the quick board and removes the piece that was there before
#[allow(dead_code)]
pub fn insert_quick_piece_into_board(
    piece: QuickPiece,
    x_coord: usize,
    y_coord: usize,
    board: &mut Vec<Vec<QuickPiece>>,
) {
    board.get_mut(x_coord).unwrap().remove(y_coord);
    board.get_mut(x_coord).unwrap().insert(y_coord, piece);
}

// @TODO maybe add checks on the x_coord or y_coord.  idk
pub fn create_board_with_piece(
    x_coord: usize,
    y_coord: usize,
    piece_type: piece_types::QuickPiece,
) -> Vec<Vec<piece_types::QuickPiece>> {
    let mut board = Vec::new();
    for _ in 0..x_coord {
        board.push(vec![
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
        ]);
    }

    let mut line_with_piece = Vec::with_capacity(8);
    for _ in 0..y_coord {
        line_with_piece.push(QuickPiece::EMPTY);
    }

    line_with_piece.push(piece_type);
    for _ in (y_coord + 1)..8 {
        line_with_piece.push(QuickPiece::EMPTY);
    }
    board.push(line_with_piece);

    for _ in (x_coord + 1)..8 {
        board.push(vec![
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
            QuickPiece::EMPTY,
        ]);
    }

    board
}

#[cfg(test)]
mod tests {
    use super::create_board_with_piece;
    use chess::piece_types::{PieceColor, QuickPiece};

    #[test]
    fn test_create_board_with_piece_0_0() {
        let board = create_board_with_piece(0, 0, QuickPiece::PIECE(PieceColor::WHITE));

        assert_eq!(board.len(), 8);
        for _ in 0..8 {
            assert_eq!(board.len(), 8)
        }
        let piece = board.get(0).unwrap().get(0).unwrap();
        //let expected_piece = QuickPiece::PIECE(PieceColor::WHITE);

        match piece {
            QuickPiece::PIECE(color) => match color {
                PieceColor::WHITE => assert!(true),
                PieceColor::BLACK => {
                    assert!(false, "Expected Piece of Color WHITE, but found BLACK")
                }
            },
            QuickPiece::EMPTY => assert!(false, "Expected Piece of color WHITE, but found EMPTY"),
            QuickPiece::KING(_) => assert!(false, "Expected Piece of color WHITE, but found KING"),
        }
    }

    #[test]
    fn test_create_board_with_king_0_0() {
        let board = create_board_with_piece(0, 0, QuickPiece::KING(PieceColor::BLACK));

        assert_eq!(board.len(), 8);
        for _ in 0..8 {
            assert_eq!(board.len(), 8)
        }
        let piece = board.get(0).unwrap().get(0).unwrap();

        match piece {
            QuickPiece::PIECE(_) => assert!(false, "Expected KING of color BLACK, but found PIECE"),
            QuickPiece::EMPTY => assert!(false, "Expected KING of color BLACK, but found EMPTY"),
            QuickPiece::KING(color) => match color {
                PieceColor::WHITE => assert!(
                    false,
                    "The color WHITE for the KING on the board is incorrect"
                ),
                PieceColor::BLACK => assert!(true),
            },
        }
    }

    #[test]
    fn test_create_board_with_king_7_7() {
        let board = create_board_with_piece(7, 7, QuickPiece::KING(PieceColor::BLACK));

        assert_eq!(
            board.len(),
            8,
            "The x-length of the board {} did not match the expected {}",
            board.len(),
            8
        );
        for _ in 0..8 {
            assert_eq!(
                board.len(),
                8,
                "The y-length of the board {} did not match the expected {}",
                board.len(),
                8
            );
        }
        let piece = board.get(7).unwrap().get(7).unwrap();

        match piece {
            QuickPiece::PIECE(_) => assert!(false, "Expected KING of color BLACK, but found PIECE"),
            QuickPiece::EMPTY => assert!(false, "Expected KING of color BLACK, but found EMPTY"),
            QuickPiece::KING(color) => match color {
                PieceColor::WHITE => assert!(
                    false,
                    "The color WHITE for the KING on the board is incorrect"
                ),
                PieceColor::BLACK => assert!(true),
            },
        }
    }

    #[test]
    fn test_create_board_with_king_3_2() {
        let board = create_board_with_piece(3, 2, QuickPiece::KING(PieceColor::BLACK));

        assert_eq!(
            board.len(),
            8,
            "The x-length of the board {} did not match the expected {}",
            board.len(),
            8
        );
        for _ in 0..8 {
            assert_eq!(
                board.len(),
                8,
                "The y-length of the board {} did not match the expected {}",
                board.len(),
                8
            );
        }
        let piece = board.get(3).unwrap().get(2).unwrap();

        match piece {
            QuickPiece::PIECE(_) => assert!(false, "Expected KING of color BLACK, but found PIECE"),
            QuickPiece::EMPTY => assert!(false, "Expected KING of color BLACK, but found EMPTY"),
            QuickPiece::KING(color) => match color {
                PieceColor::WHITE => assert!(
                    false,
                    "The color WHITE for the KING on the board is incorrect"
                ),
                PieceColor::BLACK => assert!(true),
            },
        }
    }
}
