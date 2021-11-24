use crate::piece_types::QuickPiece;
use crate::pieces::check_if_piece_on_location;

/// This function should return  true if there are any pieces in the path to the final location.
/// This does NOT include the final location
/// This function assumes that the path it's going to is a valid location for a bishop
pub fn check_if_pieces_in_path_diag(
    x_start: usize,
    y_start: usize,
    x_dest: usize,
    y_dest: usize,
    quick_board: &[Vec<QuickPiece>],
) -> bool {
    let mut x_step: isize = 1;
    let mut y_step: isize = 1;
    if x_dest < x_start {
        x_step = -1;
    }
    if y_dest < y_start {
        y_step = -1;
    }

    // @TODO This looks gross and needs a refactor
    // That -1 I believe is so that it's all positions up to the final one will need to double check
    let max = usize::max(x_dest, x_start);
    let min = usize::min(x_dest, x_start);
    for index in 1..(max - min) {
        let current_pos: (isize, isize) = (
            x_start as isize + (x_step * index as isize),
            y_start as isize + (y_step * index as isize),
        );

        let result = match quick_board
            .get(current_pos.0 as usize)
            .unwrap()
            .get(current_pos.1 as usize)
            .unwrap()
        {
            QuickPiece::PIECE(_) => true,
            QuickPiece::KING(_) => true,
            QuickPiece::EMPTY => false,
        };
        if result {
            return true;
        }
    }

    false
}
pub fn is_move_diagonal(
    x_start: usize,
    y_start: usize,
    x_coord: usize,
    y_coord: usize,
    _quick_board: &[Vec<QuickPiece>],
) -> bool {
    let x_delta = usize::max(x_coord, x_start) - usize::min(x_coord, x_start);
    let y_delta = usize::max(y_coord, y_start) - usize::min(y_coord, y_start);

    if x_delta == 0 {
        return false;
    }

    x_delta != 0 && x_delta == y_delta
}

pub fn is_move_horizontal_vertical(
    x_start: usize,
    y_start: usize,
    x_coord: usize,
    y_coord: usize,
    _quick_board: &[Vec<QuickPiece>],
) -> bool {
    let x_delta = usize::max(x_coord, x_start) - usize::min(x_coord, x_start);
    let y_delta = usize::max(y_coord, y_start) - usize::min(y_coord, y_start);
    // If it's a horizontal or vertical movement one of these will have a delta of 0.
    // If both are 0 then the piece which is still horizontal or vertical
    x_delta == 0 || y_delta == 0
}

/// This function should return  true if there are any pieces in the path to the final location.
/// This does NOT include the final location
/// This function assumes that the path it's going to is a valid location for a bishop
pub fn check_if_pieces_in_path_horizontal_vertical(
    x_start: usize,
    y_start: usize,
    x_coord: usize,
    y_coord: usize,
    quick_board: &[Vec<QuickPiece>],
) -> bool {
    let mut x_step: isize = 1;
    let mut y_step: isize = 1;
    if x_coord < x_start {
        x_step = -1;
    }
    if y_coord < y_start {
        y_step = -1;
    }

    if x_coord == x_start {
        let max = usize::max(y_coord, y_start);
        let min = usize::min(y_coord, y_start);
        for index in 1..(max - min) {
            let current_pos: (isize, isize) = (
                x_coord as isize,
                y_start as isize + (y_step * index as isize),
            );
            if check_if_piece_on_location(
                current_pos.0 as usize,
                current_pos.1 as usize,
                quick_board,
            ) {
                return true;
            }
        }
    } else if y_coord == y_start {
        let max = usize::max(x_coord, x_start);
        let min = usize::min(x_coord, x_start);
        for index in 1..(max - min) {
            let current_pos: (isize, isize) = (
                x_start as isize + (x_step * index as isize),
                y_coord as isize,
            );
            if check_if_piece_on_location(
                current_pos.0 as usize,
                current_pos.1 as usize,
                quick_board,
            ) {
                return true;
            }
        }
    }

    false
}
