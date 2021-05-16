#[derive(Debug)]
pub enum QuickPiece {
    PIECE(PieceColor),
    EMPTY,
    KING(PieceColor),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    WHITE,
    BLACK,
}
