#[derive(Debug, PartialEq)]
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

impl PieceColor {
    pub fn opposite_color(piece_color: &PieceColor) -> PieceColor {
        match piece_color {
            PieceColor::BLACK => PieceColor::WHITE,
            PieceColor::WHITE => PieceColor::BLACK,
        }
    }
}
