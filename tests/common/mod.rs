use chess::piece_types;
use chess::piece_types::QuickPiece;
use chess::pieces::bishop;

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
    use chess::pieces::bishop;

    #[test]
    fn test_create_board_with_piece_0_0() {
        let board = create_board_with_piece(0, 0, QuickPiece::PIECE(PieceColor::WHITE));

        assert_eq!(board.len(), 8);
        for _ in 0..8 {
            assert_eq!(board.len(), 8)
        }
        let piece = board.get(0).unwrap().get(0).unwrap();
        let expected_piece = QuickPiece::PIECE(PieceColor::WHITE);

        match piece {
            QuickPiece::PIECE(color) => match color {
                WHITE => assert!(true),
            },
            QuickPiece::EMPTY => assert!(false, "Expected Piece of color WHITE, but found EMPTY"),
            QuickPiece::KING(color) => {
                assert!(false, "Expected Piece of color WHITE, but found KING")
            }
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
            QuickPiece::PIECE(color) => {
                assert!(false, "Expected KING of color BLACK, but found PIECE")
            }
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
            QuickPiece::PIECE(color) => {
                assert!(false, "Expected KING of color BLACK, but found PIECE")
            }
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
            QuickPiece::PIECE(color) => {
                assert!(false, "Expected KING of color BLACK, but found PIECE")
            }
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
