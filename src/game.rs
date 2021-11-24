use crate::board::{Board, MoveError};
use crate::gui_runner::GuiRunner;
use crate::parser::parse_move;
use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::{AnyPiece, PieceMove};
use iced::{Sandbox, Settings};
use std::io;

// This function only checks the color opposing the last move.  This is because one cannot make a
// move that would put the player in check.  That means for this function to run it has already
// been checked that the king of the last_move color is not in check.
pub fn is_board_in_check(last_move: &PieceColor, board: &Board) -> bool {
    let opposing_king = match last_move {
        PieceColor::WHITE => board.black_king_position,
        PieceColor::BLACK => board.white_king_position,
    };
    match last_move {
        PieceColor::WHITE => {
            for piece in &board.live_white_pieces {
                if piece.can_move(opposing_king.0, opposing_king.1, &board.position_board) {
                    return true;
                }
            }
        }
        PieceColor::BLACK => {
            for piece in &board.live_black_pieces {
                if piece.can_move(opposing_king.0, opposing_king.1, &board.position_board) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn is_board_stale_mate(board: &mut Board) -> bool {
    if !can_any_piece_move(&PieceColor::WHITE, board)
        && !is_board_in_check(&PieceColor::WHITE, board)
    {
        return true;
    }

    if !can_any_piece_move(&PieceColor::BLACK, board)
        && !is_board_in_check(&PieceColor::WHITE, board)
    {
        return true;
    }
    false
}

pub fn is_board_draw_by_repetition(board: &mut Board) -> bool {
    let current_hash = board.get_current_hash();
    if board.board_state_hashes.contains_key(&current_hash)
        && *board.board_state_hashes.get(&current_hash).unwrap() >= 3
    {
        return true;
    }
    false
}

pub fn is_board_check_mate(last_move: &PieceColor, board: &mut Board) -> bool {
    let opposing_king_color = match last_move {
        PieceColor::WHITE => PieceColor::BLACK,
        PieceColor::BLACK => PieceColor::WHITE,
    };

    if is_board_in_check(last_move, board) && !can_any_piece_move(&opposing_king_color, board) {
        return true;
    }

    false
}

fn can_any_piece_move(piece_color: &PieceColor, game_board: &mut Board) -> bool {
    let all_possible_moves: Vec<((usize, usize), Vec<(usize, usize)>)> = match piece_color {
        PieceColor::WHITE => {
            let mut all_movable_moves = Vec::new();
            for piece in &game_board.live_white_pieces {
                let possible_moves = piece.moves_on_board();
                let mut movable_moves = Vec::with_capacity(possible_moves.capacity());

                for (x, y) in possible_moves {
                    if piece.can_move(x, y, &game_board.position_board) {
                        let (px, py) = piece.get_pos();
                        if let AnyPiece::Pawn(_) = piece {
                            if game_board.valid_en_passant("P", px, py, x, y).is_ok() {
                                movable_moves.push((x, y));
                            }
                        } else {
                            movable_moves.push((x, y));
                        }
                    }
                }
                all_movable_moves.push((piece.get_pos(), movable_moves));
            }
            all_movable_moves
        }
        PieceColor::BLACK => {
            let mut all_movable_moves = Vec::new();
            for piece in &game_board.live_black_pieces {
                let possible_moves = piece.moves_on_board();
                let mut movable_moves = Vec::with_capacity(possible_moves.capacity());

                for (x, y) in possible_moves {
                    if piece.can_move(x, y, &game_board.position_board) {
                        let (px, py) = piece.get_pos();
                        if let AnyPiece::Pawn(_) = piece {
                            if game_board.valid_en_passant("P", px, py, x, y).is_ok() {
                                movable_moves.push((x, y));
                            }
                        } else {
                            movable_moves.push((x, y));
                        }
                    }
                }
                all_movable_moves.push((piece.get_pos(), movable_moves));
            }
            all_movable_moves
            //game_board.live_black_pieces.iter().map(|x| (x.get_pos(),x.moves_on_board())).collect()
        }
    };

    //let all_possible_moves:Vec<((usize,usize),Vec<(usize,usize)>)> = live_pieces.iter().map(|x| (x.get_pos(),x.moves_on_board())).collect();
    for ((piece_pos_x, piece_pos_y), moves) in all_possible_moves {
        for possible_move in moves {
            if !will_move_be_in_check(
                piece_pos_x,
                piece_pos_y,
                possible_move.0,
                possible_move.1,
                &piece_color.clone(),
                &piece_color.clone(),
                game_board,
            ) {
                return true;
            }
        }
    }

    if game_board.can_castle_king(piece_color, 6) || game_board.can_castle_king(piece_color, 2) {
        return true;
    }

    false
}

/// While this function takes in a mutable board it will not return until the board is back to the original state
/// This move will return true if there exists a piece in the opposing live pieces vector that can move to the king_color_being_checked's location
/// This will return false otherwise
/// @TODO Think about where I want more input checks
/// @TODO this feels kind of long
pub fn will_move_be_in_check(
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
    moving_piece_color: &PieceColor,
    king_color_being_checked: &PieceColor,
    board: &mut Board,
) -> bool {
    // Remove the starting piece from quick board
    let start_piece = board
        .position_board
        .get_mut(x_start)
        .unwrap()
        .remove(y_start);

    // insert empty in the starting pieces's place
    board
        .position_board
        .get_mut(x_start)
        .unwrap()
        .insert(y_start, QuickPiece::EMPTY);

    // Remove the piece in the location the starting piece is moving to
    let end_quick_piece = board.position_board.get_mut(x_end).unwrap().remove(y_end);

    let mut live_end_piece = None;
    let mut live_end_piece_color = None;
    // This is removing the end piece from a living piece list if there was a piece at that location
    match &end_quick_piece {
        QuickPiece::KING(color) | QuickPiece::PIECE(color) => match color {
            PieceColor::WHITE => {
                for (index, piece) in board.live_white_pieces.iter().enumerate() {
                    if piece.get_pos() == (x_end, y_end) {
                        live_end_piece_color = Some(PieceColor::WHITE);
                        live_end_piece = Some(board.live_white_pieces.remove(index));
                        break;
                    }
                }
            }
            PieceColor::BLACK => {
                for (index, piece) in board.live_black_pieces.iter().enumerate() {
                    if piece.get_pos() == (x_end, y_end) {
                        live_end_piece_color = Some(PieceColor::BLACK);
                        live_end_piece = Some(board.live_black_pieces.remove(index));
                        break;
                    }
                }
            }
        },
        _ => (),
    };

    // insert the starting piece at the end location in the quick board
    board
        .position_board
        .get_mut(x_end)
        .unwrap()
        .insert(y_end, start_piece);

    // Set the starting_pieces location in the AnyPiece struct
    let live_moving_piece = board
        .find_piece_color(x_start, y_start, moving_piece_color)
        .unwrap();
    live_moving_piece.set_pos(x_end, y_end);

    if let AnyPiece::King(_) = live_moving_piece {
        match moving_piece_color {
            PieceColor::WHITE => board.white_king_position = (x_end, y_end),
            PieceColor::BLACK => board.black_king_position = (x_end, y_end),
        }
    };

    let opposing_king_pos = match king_color_being_checked {
        PieceColor::WHITE => board.white_king_position,
        PieceColor::BLACK => board.black_king_position,
    };

    let found_check = board.can_any_piece_check_king(
        opposing_king_pos.0,
        opposing_king_pos.1,
        king_color_being_checked,
    );

    // Clean up
    let start_piece = board.position_board.get_mut(x_end).unwrap().remove(y_end);
    board
        .position_board
        .get_mut(x_end)
        .unwrap()
        .insert(y_end, end_quick_piece);

    if let Some(piece) = live_end_piece {
        match live_end_piece_color.unwrap() {
            PieceColor::WHITE => board.live_white_pieces.push(piece),
            PieceColor::BLACK => board.live_black_pieces.push(piece),
        }
    };

    board
        .position_board
        .get_mut(x_start)
        .unwrap()
        .remove(y_start);
    board
        .position_board
        .get_mut(x_start)
        .unwrap()
        .insert(y_start, start_piece);

    let found_piece = board
        .find_piece_color(x_end, y_end, moving_piece_color)
        .unwrap();
    found_piece.set_pos(x_start, y_start);

    if let AnyPiece::King(king) = found_piece {
        match moving_piece_color {
            PieceColor::WHITE => board.white_king_position = king.get_pos(),
            PieceColor::BLACK => board.black_king_position = king.get_pos(),
        }
    };
    found_check
}

fn get_input_string_from_user() -> String {
    println!("Please input a move");
    let mut player_input_string = String::new();
    io::stdin()
        .read_line(&mut player_input_string)
        .expect("Failed to retrieve the player's move from the standard input");
    player_input_string
}

fn get_player_draw_response() -> bool {
    let mut draw_response = String::new();
    println!("The other player is asking for a draw.  Would you like to accept?(Y/N)");
    io::stdin()
        .read_line(&mut draw_response)
        .expect("Failed to retrieve the player's move from the standard input");

    draw_response.to_ascii_lowercase().contains('y')
}

// @TODO figure out how to maintain the parsed error
pub fn player_move(game_board: &mut Board, player_input: &str) -> Result<(), MoveError> {
    let parsed_move = parse_move(player_input);
    // play_move should be Result<(), MoveError>
    // MoveError should replace all the panics.  It should also include if one would be in check if they moved.
    match parsed_move {
        Ok(parsed_move) => game_board.play_move(parsed_move),
        // This is a horrible way to do this I bet
        Err(parsed_move_error) => Err(MoveError::new(format!("{:?}", parsed_move_error).as_str())), // There was an error getting the move
    }
}

pub fn play_game_cli() {
    // Create the game
    let mut game_board = Board::new();
    let mut game_continue_status = true;

    while game_continue_status {
        // We need to keep alternating pieces until checkmate, stalemate, or draw
        let mut valid_move = false;

        while !valid_move {
            println!(
                "CurrentMoveColor:{:?}",
                PieceColor::opposite_color(&game_board.last_move_color)
            );
            let player_input = get_input_string_from_user();
            if player_input.contains("print") {
                println!(
                    "CheckMate?:{:?}",
                    is_board_check_mate(&game_board.last_move_color.clone(), &mut game_board)
                );
                println!("currentQuickBoard:{:?}", game_board.position_board);
                println!("LiveWhite:{:?}", game_board.live_white_pieces);
                println!("LiveBlack:{:?}", game_board.live_black_pieces);
                println!("PastMoves:{:?}", game_board.played_moves);
            } else if player_input.contains("draw") {
                if get_player_draw_response() {
                    // End the game with a score of draw
                    println!("The players have agreed to a draw!");
                    game_continue_status = false;
                    valid_move = true;
                } else {
                    println!("The players failed to agree to a draw!");
                }
            } else {
                let move_result = player_move(&mut game_board, &player_input);
                match move_result {
                    Ok(_) => valid_move = true,
                    Err(error) => println!("{}", error),
                };
            }
        }

        // The current player has finished their move.
        // check for checkmate or stalemate
        let check_mate_status =
            is_board_check_mate(&game_board.last_move_color.clone(), &mut game_board);
        if check_mate_status {
            // Do something for a win for that last move color
            println!("{:?} Wins with checkmate", game_board.last_move_color);
            game_continue_status = false;
        }
    }
}

pub fn play_game_gui() {
    GuiRunner::run(Settings::default()).unwrap();
}

pub fn file_name_from_piece(any_piece: &AnyPiece, piece_color: &PieceColor) -> String {
    String::from(match piece_color {
        PieceColor::WHITE => match any_piece {
            AnyPiece::King(_) => "GreenKing.png",
            AnyPiece::Queen(_) => "GreenQueen.png",
            AnyPiece::Bishop(_) => "GreenBishop.png",
            AnyPiece::Knight(_) => "GreenKnight.png",
            AnyPiece::Rook(_) => "GreenRook.png",
            AnyPiece::Pawn(_) => "GreenPawn.png",
        },
        PieceColor::BLACK => match any_piece {
            AnyPiece::King(_) => "BlackKing.png",
            AnyPiece::Queen(_) => "BlackQueen.png",
            AnyPiece::Bishop(_) => "BlackBishop.png",
            AnyPiece::Knight(_) => "BlackKnight.png",
            AnyPiece::Rook(_) => "BlackRook.png",
            AnyPiece::Pawn(_) => "BlackPawn.png",
        },
    })
}
