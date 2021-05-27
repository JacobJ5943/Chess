use crate::game;
use crate::parser;
use crate::parser::{MoveTypes, ParsedMove};
use crate::piece_types::{PieceColor, QuickPiece};
use crate::pieces::bishop::Bishop;
use crate::pieces::king::King;
use crate::pieces::knight::Knight;
use crate::pieces::pawn::Pawn;
use crate::pieces::queen::Queen;
use crate::pieces::rook::Rook;
use crate::pieces::{AnyPiece, PieceMove};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct MoveError {
    details: String,
}

impl MoveError {
    pub(crate) fn new(msg: &str) -> MoveError {
        MoveError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MoveError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, PartialEq)]
pub struct PlayedMove {
    piece_string: String,
    starting_position: (usize, usize),
    end_position: (usize, usize),
    moving_color: PieceColor,
    promotion_piece: Option<String>,
}

impl PlayedMove {
    pub fn new(
        piece_string: String,
        starting_position: (usize, usize),
        end_position: (usize, usize),
        moving_color: PieceColor,
        promotion_piece: Option<String>,
    ) -> PlayedMove {
        PlayedMove {
            piece_string,
            starting_position,
            end_position,
            moving_color,
            promotion_piece,
        }
    }

    pub fn get_ending_pos(&self) -> (usize, usize) {
        self.end_position
    }
}

pub struct Board {
    pub position_board: Vec<Vec<QuickPiece>>,
    pub live_white_pieces: Vec<AnyPiece>,
    pub live_black_pieces: Vec<AnyPiece>,
    pub white_king_position: (usize, usize),
    pub black_king_position: (usize, usize),
    pub last_move_color: PieceColor, // @TODO with last played move being a thing this is redundant
    pub played_moves: Vec<PlayedMove>, // remove this to be the last played move
    pub board_state_hashes: HashMap<u64, usize>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            position_board: Board::create_default_position_board(),
            live_white_pieces: Board::default_live_white_pieces(),
            live_black_pieces: Board::default_live_black_pieces(),
            white_king_position: Board::default_white_king_pos(),
            black_king_position: Board::default_black_king_pos(),
            last_move_color: PieceColor::BLACK,
            played_moves: Vec::new(),
            board_state_hashes: HashMap::new(),
        };
        board.add_state_hash();
        board
    }

    fn default_live_white_pieces() -> Vec<AnyPiece> {
        vec![
            AnyPiece::Rook(Rook::new(0, 0, PieceColor::WHITE)),
            AnyPiece::Rook(Rook::new(7, 0, PieceColor::WHITE)),
            AnyPiece::Knight(Knight::new(1, 0, PieceColor::WHITE)),
            AnyPiece::Knight(Knight::new(6, 0, PieceColor::WHITE)),
            AnyPiece::Bishop(Bishop::new(2, 0, PieceColor::WHITE)),
            AnyPiece::Bishop(Bishop::new(5, 0, PieceColor::WHITE)),
            AnyPiece::Queen(Queen::new(3, 0, PieceColor::WHITE)),
            AnyPiece::King(King::new(4, 0, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(0, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(1, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(2, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(3, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(4, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(5, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(6, 1, PieceColor::WHITE)),
            AnyPiece::Pawn(Pawn::new(7, 1, PieceColor::WHITE)),
        ]
    }

    fn default_live_black_pieces() -> Vec<AnyPiece> {
        vec![
            AnyPiece::Rook(Rook::new(0, 7, PieceColor::BLACK)),
            AnyPiece::Rook(Rook::new(7, 7, PieceColor::BLACK)),
            AnyPiece::Knight(Knight::new(1, 7, PieceColor::BLACK)),
            AnyPiece::Knight(Knight::new(6, 7, PieceColor::BLACK)),
            AnyPiece::Bishop(Bishop::new(2, 7, PieceColor::BLACK)),
            AnyPiece::Bishop(Bishop::new(5, 7, PieceColor::BLACK)),
            AnyPiece::Queen(Queen::new(3, 7, PieceColor::BLACK)),
            AnyPiece::King(King::new(4, 7, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(0, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(1, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(2, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(3, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(4, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(5, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(6, 6, PieceColor::BLACK)),
            AnyPiece::Pawn(Pawn::new(7, 6, PieceColor::BLACK)),
        ]
    }

    fn is_correct_piece_type(piece: &AnyPiece, compare_type: &str) -> bool {
        let compare_type = String::from(compare_type);
        match piece {
            AnyPiece::Knight(_) => compare_type == String::from("N"),
            AnyPiece::King(_) => compare_type == String::from("K"),
            AnyPiece::Queen(_) => compare_type == String::from("Q"),
            AnyPiece::Rook(_) => compare_type == String::from("R"),
            AnyPiece::Pawn(_) => compare_type == String::from("P"),
            AnyPiece::Bishop(_) => compare_type == String::from("B"),
        }
    }

    pub fn find_start_piece_from_move(
        &mut self,
        parsed_move: &parser::ParsedMove,
    ) -> Option<&mut AnyPiece> {
        let piece_list = match self.last_move_color {
            PieceColor::BLACK => &mut self.live_white_pieces,
            PieceColor::WHITE => &mut self.live_black_pieces,
        };

        let end_x: usize = parser::parse_coordinate(&parsed_move.end_coords.0);
        let end_y: usize = parser::parse_coordinate(&parsed_move.end_coords.1);
        let mut return_value = None;
        for piece in piece_list {
            // find a piece that can move to that location and is of the corret type
            // This move must not leave the player in check
            // This move must be valid.  A pawn can only move diagonal in a take or en passant
            if piece.can_move(end_x, end_y, &self.position_board)
                && Board::is_correct_piece_type(piece, &parsed_move.piece_char)
            {
                if Board::is_correct_moving_piece(parsed_move, piece) {
                    if &String::from("P") == &parsed_move.piece_char {
                        // @TODO this could probably be a function
                        let delta_x = usize::max(end_x, piece.get_pos().0)
                            - usize::min(end_x, piece.get_pos().0);
                        match &parsed_move.move_type {
                            MoveTypes::Take => {
                                if delta_x == 1 {
                                    return_value = Some(piece);
                                }
                            }
                            MoveTypes::Move => {
                                if delta_x == 0 {
                                    return_value = Some(piece);
                                }
                            }
                            MoveTypes::Promote(_) => {
                                match self.position_board.get(end_x).unwrap().get(end_y).unwrap() {
                                    QuickPiece::EMPTY => {
                                        if delta_x == 0 {
                                            return_value = Some(piece);
                                        }
                                    }
                                    _ => {
                                        if delta_x == 1 {
                                            return_value = Some(piece);
                                        }
                                    }
                                }
                            }
                            _ => (),
                        };
                    } else {
                        return_value = Some(piece);
                    }
                }
            }
        }
        return_value
    }

    fn is_correct_moving_piece(parsed_move: &ParsedMove, piece: &AnyPiece) -> bool {
        let mut startin_x_match = true;
        let mut startin_y_match = true;
        if let Some(starting_x_coord) = &parsed_move.starting_coords.0 {
            startin_x_match =
                parser::parse_coordinate(starting_x_coord.as_str()) == piece.get_pos().0
        };
        if let Some(starting_y_coord) = &parsed_move.starting_coords.1 {
            startin_y_match =
                parser::parse_coordinate(starting_y_coord.as_str()) == piece.get_pos().1
        };

        startin_x_match && startin_y_match
    }

    // @TODO ANother method that will have errors that need to be checked
    fn remove_piece_color(
        &mut self,
        x_coord: usize,
        y_coord: usize,
        color_to_remove: &PieceColor,
    ) -> bool {
        match color_to_remove {
            PieceColor::WHITE => {
                for (piece, index) in self
                    .live_white_pieces
                    .iter()
                    .zip(0..self.live_white_pieces.len())
                {
                    if piece.get_pos() == (x_coord, y_coord) {
                        self.live_white_pieces.remove(index);
                        return true;
                    }
                }
            }
            PieceColor::BLACK => {
                for (piece, index) in self
                    .live_black_pieces
                    .iter()
                    .zip(0..self.live_black_pieces.len())
                {
                    if piece.get_pos() == (x_coord, y_coord) {
                        self.live_black_pieces.remove(index);
                        return true;
                    }
                }
            }
        };

        false
    }

    pub fn find_piece_color(
        &mut self,
        x_coord: usize,
        y_coord: usize,
        piece_color: &PieceColor,
    ) -> Option<&mut AnyPiece> {
        let piece_list = match piece_color {
            PieceColor::WHITE => &mut self.live_white_pieces,
            PieceColor::BLACK => &mut self.live_black_pieces,
        };

        for piece in piece_list {
            if piece.get_pos() == (x_coord, y_coord) {
                return Some(piece);
            }
        }
        None
    }

    pub fn can_any_piece_check_king(
        &mut self,
        x_coord: usize,
        y_coord: usize,
        king_color: &PieceColor,
    ) -> bool {
        let mut found_movement = false;
        match king_color {
            PieceColor::WHITE => {
                for piece in &self.live_black_pieces {
                    if piece.can_move(x_coord, y_coord, &self.position_board) {
                        found_movement = true;
                    }
                }
            }
            PieceColor::BLACK => {
                for piece in &self.live_white_pieces {
                    if piece.can_move(x_coord, y_coord, &self.position_board) {
                        found_movement = true;
                    }
                }
            }
        }
        found_movement
    }

    pub fn default_black_king_pos() -> (usize, usize) {
        (4, 7)
    }
    pub fn default_white_king_pos() -> (usize, usize) {
        (4, 0)
    }
    pub fn create_default_position_board() -> Vec<Vec<QuickPiece>> {
        // Cargo fmt is so gross
        vec![
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::KING(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::KING(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
            vec![
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::PIECE(PieceColor::WHITE),
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::EMPTY,
                QuickPiece::PIECE(PieceColor::BLACK),
                QuickPiece::PIECE(PieceColor::BLACK),
            ],
        ]
    }

    fn move_in_quick_board(
        &mut self,
        moving_x: usize,
        moving_y: usize,
        _moving_piece_color: &PieceColor, // @TODO remove this from the funciton definition then probably
        end_x: usize,
        end_y: usize,
    ) {
        let starting_piece = self
            .position_board
            .get_mut(moving_x)
            .unwrap()
            .remove(moving_y);
        self.position_board
            .get_mut(moving_x)
            .unwrap()
            .insert(moving_y, QuickPiece::EMPTY);
        self.position_board.get_mut(end_x).unwrap().remove(end_y);
        self.position_board
            .get_mut(end_x)
            .unwrap()
            .insert(end_y, starting_piece);
    }

    fn move_piece(
        &mut self,
        piece_symbol: String,
        moving_x: usize,
        moving_y: usize,
        moving_piece_color: &PieceColor,
        end_x: usize,
        end_y: usize,
        promotion_piece: Option<&str>,
    ) {
        if let Ok(result) =
            self.valid_en_passant(piece_symbol.as_str(), moving_x, moving_y, end_x, end_y)
        {
            if result {
                let last_move = self.played_moves.last_mut().unwrap();
                let last_move_color = last_move.moving_color.clone();
                let last_move_end_pos = last_move.end_position.clone();
                println!("removing the piece");
                self.remove_piece_color(last_move_end_pos.0, last_move_end_pos.1, &last_move_color);

                self.position_board
                    .get_mut(last_move_end_pos.0)
                    .unwrap()
                    .remove(last_move_end_pos.1);
                self.position_board
                    .get_mut(last_move_end_pos.0)
                    .unwrap()
                    .insert(last_move_end_pos.1, QuickPiece::EMPTY);
            }
        };

        self.move_in_quick_board(moving_x, moving_y, moving_piece_color, end_x, end_y);

        let color_being_taken = match moving_piece_color {
            PieceColor::WHITE => PieceColor::BLACK,
            PieceColor::BLACK => PieceColor::WHITE,
        };

        let _removed = self.remove_piece_color(end_x, end_y, &color_being_taken);

        // Set the piece's new position
        let moving_piece = self
            .find_piece_color(moving_x, moving_y, moving_piece_color)
            .unwrap();
        &moving_piece.set_pos(end_x, end_y);

        match moving_piece {
            AnyPiece::King(_king) => {
                match moving_piece_color {
                    PieceColor::BLACK => self.black_king_position = (end_x, end_y),
                    PieceColor::WHITE => self.white_king_position = (end_x, end_y),
                };
            }
            AnyPiece::Pawn(_) => {
                if end_y == 7 {
                    self.promote_pawn_at(
                        end_x,
                        end_y,
                        moving_piece_color,
                        promotion_piece.unwrap(),
                    );
                }
            }
            _ => (),
        };

        match promotion_piece {
            Some(promotion_piece) => self.played_moves.push(PlayedMove::new(
                piece_symbol,
                (moving_x, moving_y),
                (end_x, end_y),
                moving_piece_color.clone(),
                Some(String::from(promotion_piece)),
            )),
            None => self.played_moves.push(PlayedMove::new(
                piece_symbol,
                (moving_x, moving_y),
                (end_x, end_y),
                moving_piece_color.clone(),
                None,
            )),
        };

        self.add_state_hash();
    }

    pub fn add_state_hash(&mut self) {
        self.live_white_pieces.sort();
        self.live_black_pieces.sort();
        let mut hasher = DefaultHasher::new();
        self.live_black_pieces.hash(&mut hasher);
        self.live_white_pieces.hash(&mut hasher);
        let hash = hasher.finish();
        if self.board_state_hashes.contains_key(&hash) {
            let current_count = self.board_state_hashes.get_mut(&hash).unwrap().clone();
            self.board_state_hashes.insert(hash, current_count + 1);
        } else {
            self.board_state_hashes.insert(hash, 1);
        }
    }

    pub fn get_current_hash(&mut self) -> u64 {
        self.live_white_pieces.sort();
        self.live_black_pieces.sort();
        let mut hasher = DefaultHasher::new();
        self.live_black_pieces.hash(&mut hasher);
        self.live_white_pieces.hash(&mut hasher);
        hasher.finish()
    }

    // @TODO this would be another prime function for adding Result to
    fn promote_pawn_at(
        &mut self,
        x_coord: usize,
        y_coord: usize,
        pawn_color: &PieceColor,
        promotion_piece: &str,
    ) {
        // first remove the pawn that is at that location
        self.remove_piece_color(x_coord, y_coord, pawn_color);
        let promoted_piece = match promotion_piece {
            "N" => AnyPiece::Knight(Knight::new(x_coord, y_coord, pawn_color.clone())),
            "Q" => AnyPiece::Queen(Queen::new(x_coord, y_coord, pawn_color.clone())),
            "B" => AnyPiece::Bishop(Bishop::new(x_coord, y_coord, pawn_color.clone())),
            "R" => AnyPiece::Rook(Rook::new(x_coord, y_coord, pawn_color.clone())),
            _ => panic!("AAAAAAAAAAAAAA in promoting pawn "),
        };

        match pawn_color {
            PieceColor::WHITE => self.live_white_pieces.push(promoted_piece),
            PieceColor::BLACK => self.live_black_pieces.push(promoted_piece),
        };
    }

    // This will return false as long as this is not a pawn attempting and failing to make an en passant
    // If it's a successful en passant it will return true.
    // if it's a failed en passant it will return an error
    // @ TODO Fix this at some point because this is a smelly return value
    pub fn valid_en_passant(
        &self,
        //parsed_move: &ParsedMove,
        moving_piece_symbol: &str,
        start_x: usize,
        _start_y: usize,
        end_x: usize,
        end_y: usize,
    ) -> Result<bool, MoveError> {
        if (7, 4) == (start_x, _start_y) && (6, 5) == (end_x, end_y) {
            println!("break");
            println!(
                "validEn:{:?}",
                self.position_board.get(end_x).unwrap().get(end_y).unwrap()
            );
        };

        if let "P" = moving_piece_symbol {
            if let QuickPiece::EMPTY = self.position_board.get(end_x).unwrap().get(end_y).unwrap() {
                let delta_x = usize::max(start_x, end_x) - usize::min(start_x, end_x);
                if delta_x == 1 {
                    if end_y == 5 || end_y == 2 {
                        if let Some(last_played_move) = self.played_moves.last() {
                            if let "P" = last_played_move.piece_string.as_str() {
                                if last_played_move.starting_position.0
                                    == last_played_move.end_position.0
                                    && last_played_move.starting_position.0 == end_x
                                {
                                    if usize::max(
                                        last_played_move.starting_position.1,
                                        last_played_move.end_position.1,
                                    ) - 1
                                        == end_y
                                    {
                                        if usize::max(
                                            last_played_move.starting_position.1,
                                            last_played_move.end_position.1,
                                        ) - 2
                                            == usize::min(
                                                last_played_move.starting_position.1,
                                                last_played_move.end_position.1,
                                            )
                                        {
                                            return Ok(true);
                                        } else {
                                            return Err(MoveError::new("Pawn tried to take to empty location without valid en passant. last pawn did not move 2 spaces"));
                                        }
                                    } else {
                                        return Err(MoveError::new("Pawn tried to take to empty location without valid en passant. Moving pawn not in the middle spot"));
                                    }
                                } else {
                                    return Err(MoveError::new("Pawn tried to take to empty location without valid en passant. Moving pawn not on last moved x location"));
                                }
                            } else {
                                return Err(MoveError::new("Pawn tried to take to empty location without valid en passant. The last moved piece is not a pawn"));
                            }
                        } else {
                            return Err(MoveError::new("Pawn tried to take to empty location without valid en passant. There is no last move"));
                        }
                    } else {
                        return Err(MoveError::new(
                            "Pawn tried to take to empty location without valid en passant.",
                        ));
                    }
                } else if delta_x == 0 {
                    return Ok(false);
                }
            }
        }
        // This isn't pertaining to the pawn so it doesn't matter
        Ok(false)
    }
    // Right now this I am assuming that this function is only used by my tests or after a move has been deemed valid
    // @TODO Maybe add if move says check or check mate make that check too
    pub fn play_move(&mut self, parsed_move: ParsedMove) -> Result<(), MoveError> {
        let current_move_color = PieceColor::opposite_color(&self.last_move_color);

        match &parsed_move.move_type {
            MoveTypes::Castle(king_end_x) => {
                self.castle_king(&current_move_color, *king_end_x);
            }
            _ => {
                let moving_piece = match self.find_start_piece_from_move(&parsed_move) {
                    None => {
                        return Err(MoveError::new(
                            format!(
                                "The moving piece could not be found for the move {:?}",
                                &parsed_move
                            )
                            .as_str(),
                        ));
                    }
                    Some(piece) => piece,
                };

                let (moving_x, moving_y) = moving_piece.get_pos();
                let end_x = parser::parse_coordinate(&parsed_move.end_coords.0);
                let end_y = parser::parse_coordinate(&parsed_move.end_coords.1);
                if !game::will_move_be_in_check(
                    moving_x,
                    moving_y,
                    end_x,
                    end_y,
                    &current_move_color,
                    &current_move_color,
                    self,
                ) {
                    // If this is an invalid en passant it will return an error  which ? will then bubble up and return here
                    self.valid_en_passant(
                        &parsed_move.piece_char,
                        moving_x,
                        moving_y,
                        end_x,
                        end_y,
                    )?;

                    if let MoveTypes::Promote(piece_promote) = parsed_move.move_type {
                        self.move_piece(
                            parsed_move.piece_char,
                            moving_x,
                            moving_y,
                            &current_move_color,
                            end_x,
                            end_y,
                            Some(piece_promote.as_str()),
                        );
                    } else {
                        self.move_piece(
                            parsed_move.piece_char,
                            moving_x,
                            moving_y,
                            &current_move_color,
                            end_x,
                            end_y,
                            None,
                        );
                    }
                } else {
                    return Err(MoveError::new(
                        format!(
                            "This move is in valid.  You are in check if you do the move {:?}",
                            &parsed_move
                        )
                        .as_str(),
                    ));
                }
            }
        };
        self.last_move_color = PieceColor::opposite_color(&self.last_move_color);
        Ok(())
    }

    pub fn can_castle_king(&mut self, king_color: &PieceColor, king_x_end: usize) -> bool {
        let mut can_castle = false;
        // 1. The king must not have moved
        let king_coords = match king_color {
            PieceColor::WHITE => self.white_king_position,
            PieceColor::BLACK => self.black_king_position,
        };

        let king_has_moved = match self.find_piece_color(king_coords.0, king_coords.1, king_color) {
            Some(king) => match king {
                AnyPiece::King(king) => king.get_has_moved(),
                _ => panic!("This should always be a king"),
            },
            None => panic!("This should always be a king"),
        };

        if !king_has_moved {
            // @TODO feels like I could replace this match with something cleaner.
            let check_rooks_and_empty = match king_x_end {
                6 => {
                    let rook = self.find_piece_color(7, king_coords.1, king_color);
                    let rook_has_not_moved = match rook {
                        Some(rook) => match rook {
                            AnyPiece::Rook(rook) => !rook.get_has_moved(),
                            _ => false,
                        },
                        None => false,
                    };
                    let no_pieces_in_path = self
                        .position_board
                        .get(5)
                        .unwrap()
                        .get(king_coords.1)
                        .unwrap()
                        == &QuickPiece::EMPTY
                        && self
                            .position_board
                            .get(6)
                            .unwrap()
                            .get(king_coords.1)
                            .unwrap()
                            == &QuickPiece::EMPTY;

                    rook_has_not_moved && no_pieces_in_path
                }
                2 => {
                    let rook = self.find_piece_color(0, king_coords.1, king_color);
                    let rook_has_not_moved = match rook {
                        Some(rook) => match rook {
                            AnyPiece::Rook(rook) => !rook.get_has_moved(),
                            _ => false,
                        },
                        None => false,
                    };
                    let no_pieces_in_path = self
                        .position_board
                        .get(2)
                        .unwrap()
                        .get(king_coords.1)
                        .unwrap()
                        == &QuickPiece::EMPTY
                        && self
                            .position_board
                            .get(3)
                            .unwrap()
                            .get(king_coords.1)
                            .unwrap()
                            == &QuickPiece::EMPTY;
                    rook_has_not_moved && no_pieces_in_path
                }
                _ => false,
            };
            if check_rooks_and_empty {
                // Then check if the king would be in check for those give positions
                can_castle = match king_x_end {
                    6 => {
                        let space1_check = !game::will_move_be_in_check(
                            king_coords.0,
                            king_coords.1,
                            king_x_end,
                            king_coords.1,
                            king_color,
                            king_color,
                            self,
                        );
                        let space2_check = !game::will_move_be_in_check(
                            king_coords.0,
                            king_coords.1,
                            5,
                            king_coords.1,
                            king_color,
                            king_color,
                            self,
                        );
                        space1_check && space2_check
                    }
                    2 => {
                        let space1_check = !game::will_move_be_in_check(
                            king_coords.0,
                            king_coords.1,
                            king_x_end,
                            king_coords.1,
                            king_color,
                            king_color,
                            self,
                        );
                        let space2_check = !game::will_move_be_in_check(
                            king_coords.0,
                            king_coords.1,
                            3,
                            king_coords.1,
                            king_color,
                            king_color,
                            self,
                        );
                        space1_check && space2_check
                    }
                    _ => false, // @TODO THis may be a mistake
                }
            }
        }

        can_castle
    }

    fn castle_king(&mut self, king_color: &PieceColor, king_x_end: usize) -> bool {
        if self.can_castle_king(king_color, king_x_end) {
            // Move the king to king_x_end
            let (king_x, king_y) = match king_color {
                PieceColor::WHITE => self.white_king_position,
                PieceColor::BLACK => self.black_king_position,
            };
            self.move_piece(
                String::from("K"),
                king_x,
                king_y,
                king_color,
                king_x_end,
                king_y,
                None,
            );
            if king_x_end == 6 {
                self.move_piece(String::from("K"), 7, king_y, king_color, 5, king_y, None);
            } else {
                self.move_piece(String::from("K"), 0, king_y, king_color, 3, king_y, None);
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::piece_types::PieceColor;
    use crate::pieces::king::King;
    use crate::pieces::rook::Rook;

    #[test]
    fn test_can_castle_white_only_no_moves() {
        // @TODO These need to be implemented
        let _rook = Rook::new(0, 0, PieceColor::WHITE);
        let _king = King::new(0, 4, PieceColor::WHITE);
    }
}
