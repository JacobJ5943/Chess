use crate::piece_types::PieceColor;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::fmt;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub struct ParsedMove {
    pub piece_char: String,
    pub starting_coords: (Option<String>, Option<String>),
    pub end_coords: (String, String),
    pub move_type: MoveTypes,
    pub check_or_checkmate: CheckOrCheckMate,
}

impl ParsedMove {
    pub fn new(
        piece_char: String,
        starting_coords: (Option<String>, Option<String>),
        end_coords: (String, String),
        move_type: MoveTypes,
        check_or_checkmate: CheckOrCheckMate,
    ) -> ParsedMove {
        ParsedMove {
            piece_char,
            starting_coords,
            end_coords,
            move_type,
            check_or_checkmate,
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum MoveTypes {
    Move,
    Take,
    Promote(String),
    Castle(usize), // This usize is going to be the end x position for the king
    FinalResult(GameResult),
}

// @TODO add stalemate eventually
#[derive(Debug, Eq, PartialEq)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw,
}

impl GameResult {
    pub fn from_string(input_string: &String) -> GameResult {
        match input_string.as_str() {
            "1-0" => GameResult::WhiteWin,
            "0-1" => GameResult::BlackWin,
            "1/2-1/2" => GameResult::Draw,
            _ => panic!("Error parsing gameResults from string"), // @TODO will need a way to remove the panic in the future.
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CheckOrCheckMate {
    Check,
    CheckMate,
    Neither,
}

pub fn parse_game_moves(
    game_string: String,
) -> Vec<(
    String,
    Result<ParsedMove, ParseError>,
    Result<ParsedMove, ParseError>,
)> {
    // Then I want to move in sets of 3
    let mut moves_vector = Vec::new();
    let moves: Vec<String> = game_string
        .split_whitespace()
        .map(|x| String::from(x))
        .collect();

    for index in (0..moves.len() / 3)
        .map(|x| x * 3)
        .filter(|x| *x < moves.len())
    {
        if moves.len() - 1 < index + 2 {
            continue;
        }

        let move_number: String = String::from(moves.get(index).unwrap());
        moves_vector.push((
            move_number,
            parse_move(moves.get(index + 1).unwrap()),
            parse_move(moves.get(index + 2).unwrap()),
        ));
    }

    moves_vector
}

pub fn parse_move(move_string: &str) -> Result<ParsedMove, ParseError> {
    let move_string = String::from(move_string);
    let mut characters = move_string.chars();
    match characters.next().unwrap() {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => parse_pawn_move(move_string),
        'K' | 'Q' | 'R' | 'B' | 'N' => parse_piece_move(move_string),
        'O' => parse_castle(move_string),
        '1' | '0' => parse_final_score(move_string),
        _ => Err(ParseError::new("Move did not start with a piece")),
    }
}

fn parse_final_score(move_string: String) -> Result<ParsedMove, ParseError> {
    Ok(ParsedMove::new(
        String::from(""),
        (None, None),
        (String::from(""), String::from("")),
        MoveTypes::FinalResult(GameResult::from_string(&move_string)),
        check_for_check_or_mate(&move_string),
    ))
}

fn parse_castle(move_string: String) -> Result<ParsedMove, ParseError> {
    let characters: Vec<char> = move_string.chars().collect();
    if characters.len() == 4 {
        // We know it's a castle king side and a check or checkmate
    }
    match characters.len() {
        3 => Ok(ParsedMove::new(
            String::from("K"),
            (None, None),
            (String::from(""), String::from("")),
            MoveTypes::Castle(6),
            CheckOrCheckMate::Neither,
        )),
        4 => Ok(ParsedMove::new(
            String::from("K"),
            (None, None),
            (String::from(""), String::from("")),
            MoveTypes::Castle(6),
            check_for_check_or_mate(&move_string),
        )),
        5 => Ok(ParsedMove::new(
            String::from("K"),
            (None, None),
            (String::from(""), String::from("")),
            MoveTypes::Castle(2),
            CheckOrCheckMate::Neither,
        )),
        6 => Ok(ParsedMove::new(
            String::from("K"),
            (None, None),
            (String::from(""), String::from("")),
            MoveTypes::Castle(2),
            check_for_check_or_mate(&move_string),
        )),
        _ => Err(ParseError::new(&format!(
            "String that starting with O was not a castling length move:{:?}",
            &move_string
        ))),
    }
}

fn parse_piece_move(move_string: String) -> Result<ParsedMove, ParseError> {
    let characters: Vec<char> = move_string.chars().collect();

    match characters.get(1).unwrap() {
        'x' => Ok(ParsedMove::new(
            characters.get(0).unwrap().to_string(),
            (None, None),
            (
                characters.get(2).unwrap().to_string(),
                characters.get(3).unwrap().to_string(),
            ),
            MoveTypes::Take,
            check_for_check_or_mate(&move_string),
        )),
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => match characters.get(2).unwrap() {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => Ok(ParsedMove::new(
                characters.get(0).unwrap().to_string(),
                (Some(characters.get(1).unwrap().to_string()), None),
                (
                    characters.get(2).unwrap().to_string(),
                    characters.get(3).unwrap().to_string(),
                ),
                MoveTypes::Move,
                check_for_check_or_mate(&move_string),
            )),
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => match characters.get(3) {
                Some('x') => Ok(ParsedMove::new(
                    characters.get(0).unwrap().to_string(),
                    (
                        Some(characters.get(1).unwrap().to_string()),
                        Some(characters.get(2).unwrap().to_string()),
                    ),
                    (
                        characters.get(4).unwrap().to_string(),
                        characters.get(5).unwrap().to_string(),
                    ),
                    MoveTypes::Take,
                    check_for_check_or_mate(&move_string),
                )),
                Some('a') | Some('b') | Some('c') | Some('d') | Some('e') | Some('f')
                | Some('g') | Some('h') => Ok(ParsedMove::new(
                    characters.get(0).unwrap().to_string(),
                    (
                        Some(characters.get(1).unwrap().to_string()),
                        Some(characters.get(2).unwrap().to_string()),
                    ),
                    (
                        characters.get(3).unwrap().to_string(),
                        characters.get(4).unwrap().to_string(),
                    ),
                    MoveTypes::Move,
                    check_for_check_or_mate(&move_string),
                )),
                Some('#') | Some('+') | None => Ok(ParsedMove::new(
                    characters.get(0).unwrap().to_string(),
                    (None, None),
                    (
                        characters.get(1).unwrap().to_string(),
                        characters.get(2).unwrap().to_string(),
                    ),
                    MoveTypes::Move,
                    check_for_check_or_mate(&move_string),
                )),
                _ => Ok(ParsedMove::new(
                    characters.get(0).unwrap().to_string(),
                    (None, None),
                    (
                        characters.get(1).unwrap().to_string(),
                        characters.get(2).unwrap().to_string(),
                    ),
                    MoveTypes::Move,
                    check_for_check_or_mate(&move_string),
                )),
            },
            'x' => Ok(ParsedMove::new(
                characters.get(0).unwrap().to_string(),
                (Some(characters.get(1).unwrap().to_string()), None),
                (
                    characters.get(3).unwrap().to_string(),
                    characters.get(4).unwrap().to_string(),
                ),
                MoveTypes::Take,
                check_for_check_or_mate(&move_string),
            )),
            _ => Err(ParseError::new("no")),
        },
        _ => Err(ParseError::new("oh no")),
    }
}

pub fn parse_coordinate(coordinate: &str) -> usize {
    match coordinate {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        "1" => 0,
        "2" => 1,
        "3" => 2,
        "4" => 3,
        "5" => 4,
        "6" => 5,
        "7" => 6,
        "8" => 7,
        _ => panic!(format!("Could not parse coordinate {}", coordinate)),
    }
}

fn parse_pawn_move(move_string: String) -> Result<ParsedMove, ParseError> {
    let characters: Vec<char> = move_string.chars().collect();
    match characters.get(1).unwrap() {
        '2' | '3' | '4' | '5' | '6' | '7' => Ok(ParsedMove::new(
            String::from("P"),
            (Some(characters.get(0).unwrap().to_string()), None),
            (
                characters.get(0).unwrap().to_string(),
                characters.get(1).unwrap().to_string(),
            ),
            MoveTypes::Move,
            check_for_check_or_mate(&move_string),
        )),
        'x' => match characters.get(4) {
            Some('=') => Ok(ParsedMove::new(
                String::from("P"),
                (Some(characters.get(0).unwrap().to_string()), None),
                (
                    characters.get(2).unwrap().to_string(),
                    characters.get(3).unwrap().to_string(),
                ),
                MoveTypes::Promote(characters.get(5).unwrap().to_string()),
                check_for_check_or_mate(&move_string),
            )),
            _ => Ok(ParsedMove::new(
                String::from("P"),
                (Some(characters.get(0).unwrap().to_string()), None),
                (
                    characters.get(2).unwrap().to_string(),
                    characters.get(3).unwrap().to_string(),
                ),
                MoveTypes::Take,
                check_for_check_or_mate(&move_string),
            )),
        },
        '1' | '8' => match characters.get(2).unwrap() {
            'Q' | 'R' | 'B' | 'N' => Ok(ParsedMove::new(
                String::from("P"),
                (None, None),
                (
                    characters.get(0).unwrap().to_string(),
                    characters.get(1).unwrap().to_string(),
                ),
                MoveTypes::Promote(characters.get(2).unwrap().to_string()),
                check_for_check_or_mate(&move_string),
            )),
            '=' => Ok(ParsedMove::new(
                String::from("P"),
                (None, None),
                (
                    characters.get(0).unwrap().to_string(),
                    characters.get(1).unwrap().to_string(),
                ),
                MoveTypes::Promote(characters.get(3).unwrap().to_string()),
                check_for_check_or_mate(&move_string),
            )),
            _ => Err(ParseError::new("This is not okay")),
        },
        _ => Err(ParseError::new("This is not okay")),
    }
}
fn check_for_check_or_mate(move_string: &str) -> CheckOrCheckMate {
    match String::from(move_string).pop().unwrap() {
        '+' => CheckOrCheckMate::Check,
        '#' => CheckOrCheckMate::CheckMate,
        _ => CheckOrCheckMate::Neither,
    }
}
#[cfg(test)]
mod tests {
    use crate::parser::CheckOrCheckMate::CheckMate;
    use crate::parser::{parse_game_moves, parse_move, CheckOrCheckMate, MoveTypes, ParsedMove};

    // Pawn moves

    #[test]
    fn test_single_pawn_move_forward() {
        let result_move = parse_move("e4").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("P"),
                (Some("e".to_string()), None),
                ("e".to_string(), "4".to_string()),
                MoveTypes::Move,
                CheckOrCheckMate::Neither
            ),
            "This should be a pawn moving from the e file to the e file"
        );
    }

    #[test]
    fn test_single_pawn_take() {
        let result_move = parse_move("exd4").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("P"),
                (Some("e".to_string()), None),
                ("d".to_string(), "4".to_string()),
                MoveTypes::Take,
                CheckOrCheckMate::Neither
            ),
            "This should be a pawn taking from the e file to the d file"
        );
    }
    #[test]
    fn test_single_pawn_take_to_promote() {
        let result_move = parse_move("exd8=N").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("P"),
                (Some("e".to_string()), None),
                ("d".to_string(), "8".to_string()),
                MoveTypes::Promote("N".to_string()),
                CheckOrCheckMate::Neither
            ),
            "This should be a pawn taking from the e file to the d file then promoting to knight"
        );
    }

    #[test]
    fn test_single_piece_move() {
        let result_move = parse_move("Nd3").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("N"),
                (None, None),
                ("d".to_string(), "3".to_string()),
                MoveTypes::Move,
                CheckOrCheckMate::Neither
            ),
            "A knight moving to d3"
        );
    }

    #[test]
    fn test_single_piece_move_specified_file() {
        let result_move = parse_move("Ned3").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("N"),
                (Some("e".to_string()), None),
                ("d".to_string(), "3".to_string()),
                MoveTypes::Move,
                CheckOrCheckMate::Neither
            ),
            "A knight moving to d3"
        );
    }

    #[test]
    fn test_single_piece_move_specified_file_and_rank() {
        let result_move = parse_move("Ne7d3").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("N"),
                (Some("e".to_string()), Some("7".to_string())),
                ("d".to_string(), "3".to_string()),
                MoveTypes::Move,
                CheckOrCheckMate::Neither
            ),
            "A knight moving to d3 from e7"
        );
    }

    #[test]
    fn test_single_piece_take() {
        let result_move = parse_move("Nxd3").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("N"),
                (None, None),
                ("d".to_string(), "3".to_string()),
                MoveTypes::Take,
                CheckOrCheckMate::Neither
            ),
            "A knight taking at d3"
        );
    }

    #[test]
    fn test_single_piece_take_specified_file() {
        let result_move = parse_move("Nexd3").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("N"),
                (Some("e".to_string()), None),
                ("d".to_string(), "3".to_string()),
                MoveTypes::Take,
                CheckOrCheckMate::Neither
            ),
            "A knight taking at d3"
        );
    }

    #[test]
    fn test_single_piece_take_specified_file_and_rank() {
        let result_move = parse_move("Ne7xd3").unwrap();
        assert_eq!(
            result_move,
            ParsedMove::new(
                String::from("N"),
                (Some("e".to_string()), Some("7".to_string())),
                ("d".to_string(), "3".to_string()),
                MoveTypes::Take,
                CheckOrCheckMate::Neither
            ),
            "A knight taking at d3 from e7"
        );
    }

    #[test]
    fn test_parse_game() {
        //https://lichess.org/m45sueue
        let result = parse_game_moves("1. e4 e6 2. d4 d5 3. e5 c5 4. c3 Ne7 5. f4 Nbc6 6. Nf3 cxd4 7. cxd4 Nf5 8. g4 Nfe7 9. Nc3 Bd7 10. Bd3 Nb4 11. O-O Ng6 12. a3 Nxd3 13. Qxd3 Be7 14. f5 exf5 15. gxf5 Nf8 16. Nxd5 g5 17. f6 g4 18. fxe7 Qa5 19. exf8=Q+ Kxf8 20. Bh6+ Ke8 21. Nf6+ Ke7 22. Nd2 Be6 23. Bg5 Kf8 24. Nfe4 h6 25. Bh4 Qb6 26. Nc5 Bd5 27. b4 Rc8 28. Nd7+ Nd7+".to_string());
    }
}
