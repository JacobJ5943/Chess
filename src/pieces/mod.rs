pub mod bishop;
pub mod king;
pub mod knight;
mod movement;
pub mod pawn;
pub mod queen;
pub mod rook;

use super::piece_types;
use crate::piece_types::QuickPiece;
use crate::pieces::king::King;
use crate::pieces::queen::Queen;
use crate::pieces::bishop::Bishop;
use crate::pieces::rook::Rook;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;

// Calling it PieceMove because I don't want to bother with if Move is already a trait or something
// silly
pub trait PieceMove {
    fn can_move(
        &self,
        x_coord: usize,
        y_coord: usize,
        quick_board: &Vec<Vec<piece_types::QuickPiece>>,
    ) -> bool;
    fn moves_on_board(&self) -> Vec<(usize, usize)>;
}

pub enum AnyPiece {
    King(King),
    Queen(Queen),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Pawn(Pawn)
}

impl PieceMove for AnyPiece {
    fn can_move(&self, x_coord: usize, y_coord: usize, quick_board: &Vec<Vec<QuickPiece>>) -> bool {
        match self {
           AnyPiece::King(ref king)  => king.can_move(x_coord,y_coord,quick_board),
            AnyPiece::Queen(ref queen)  => queen.can_move(x_coord,y_coord,quick_board),
            AnyPiece::Bishop(ref bishop)  => bishop.can_move(x_coord,y_coord,quick_board),
            AnyPiece::Knight(ref knight)  => knight.can_move(x_coord,y_coord,quick_board),
            AnyPiece::Rook(ref rook)  => rook.can_move(x_coord,y_coord,quick_board),
            AnyPiece::Pawn(ref pawn)  => pawn.can_move(x_coord,y_coord,quick_board),
        }
    }

    fn moves_on_board(&self) -> Vec<(usize, usize)> {
        match self {
            AnyPiece::King(ref king)  => king.moves_on_board(),
            AnyPiece::Queen(ref queen)  => queen.moves_on_board(),
            AnyPiece::Bishop(ref bishop)  => bishop.moves_on_board(),
            AnyPiece::Knight(ref knight)  => knight.moves_on_board(),
            AnyPiece::Rook(ref rook)  => rook.moves_on_board(),
            AnyPiece::Pawn(ref pawn)  => pawn.moves_on_board()
        }
    }
}

pub fn coord_on_board(
    x_coord: usize,
    y_coord: usize,
    quick_board: &Vec<Vec<piece_types::QuickPiece>>,
) -> bool {
    let y_length = match quick_board.get(0) {
        Some(row) => row.len(),
        None => return false, // @TODO Look into if this is rusty or crusty
    };

    if x_coord >= quick_board.len() || y_coord >= y_length {
        return false;
    }
    true
}

fn check_if_piece_on_location(
    x_coord: usize,
    y_coord: usize,
    quick_board: &Vec<Vec<piece_types::QuickPiece>>,
) -> bool {
    let piece = quick_board.get(x_coord).unwrap().get(y_coord).unwrap();
    match piece {
        piece_types::QuickPiece::PIECE(color) => true,
        piece_types::QuickPiece::KING(color) => true,
        _ => false,
    }
}
