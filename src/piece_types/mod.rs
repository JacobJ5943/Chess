#[derive(Debug)]
pub enum QuickPiece {
    PIECE(PieceColor),
    EMPTY,
    KING(PieceColor),
}

#[derive(Debug, PartialEq)]
pub enum PieceColor {
    WHITE,
    BLACK,
}

