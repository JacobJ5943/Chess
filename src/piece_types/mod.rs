#[derive(Debug)]
pub enum QuickPiece {
    PIECE(PieceColor),
    EMPTY,
    KING(PieceColor),
}

#[derive(Debug)]
pub enum PieceColor {
    WHITE,
    BLACK,
}
