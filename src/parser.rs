use std::borrow::BorrowMut;
use std::error::Error;
use std::fmt;
use std::str::Chars;

#[derive(Debug)]
struct ParseError {
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
enum MoveTypes {
    Move,
    Take,
    Promote(String), // TODO Change this to something other than string later
}

#[derive(Debug, Eq, PartialEq)]
enum CheckOrCheckMate {
    Check,
    CheckMate,
    Neither,
}

fn parse_move(
    move_string: &str,
) -> Result<
    (
        String,
        (Option<String>, Option<String>),
        (String, String),
        MoveTypes,
        CheckOrCheckMate,
    ),
    ParseError,
> {
    let move_string = String::from(move_string);
    let mut characters = move_string.chars();
    match characters.next().unwrap() {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => parse_pawn_move(move_string),
        'K' | 'Q' | 'R' | 'B' | 'N' => parse_piece_move(move_string),
        _ => Err(ParseError::new("Move did not start with a piece")),
    }
}

fn parse_piece_move(
    move_string: String,
) -> Result<
    (
        String,
        (Option<String>, Option<String>),
        (String, String),
        MoveTypes,
        CheckOrCheckMate,
    ),
    ParseError,
> {
    let characters: Vec<char> = move_string.chars().collect();

    match characters.get(1).unwrap() {
        'x' => Ok((
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
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => Ok((
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
                Some('x') => Ok((
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
                | Some('g') | Some('h') => Ok((
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
                None => Ok((
                    characters.get(0).unwrap().to_string(),
                    (None, None),
                    (
                        characters.get(1).unwrap().to_string(),
                        characters.get(2).unwrap().to_string(),
                    ),
                    MoveTypes::Move,
                    check_for_check_or_mate(&move_string),
                )),
                _ => Err(ParseError::new("oh no")),
            },
            'x' => Ok((
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
        'O' => Err(ParseError::new("CASTLE")),
        _ => Err(ParseError::new("oh no")),
    }
}

fn parse_pawn_move(
    move_string: String,
) -> Result<
    (
        String,
        (Option<String>, Option<String>),
        (String, String),
        MoveTypes,
        CheckOrCheckMate,
    ),
    ParseError,
> {
    let characters: Vec<char> = move_string.chars().collect();
    match characters.get(1).unwrap() {
        '2' | '3' | '4' | '5' | '6' | '7' => Ok((
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
            Some('=') => Ok((
                String::from("P"),
                (Some(characters.get(0).unwrap().to_string()), None),
                (
                    characters.get(2).unwrap().to_string(),
                    characters.get(3).unwrap().to_string(),
                ),
                MoveTypes::Promote(characters.get(5).unwrap().to_string()),
                check_for_check_or_mate(&move_string),
            )),
            _ => Ok((
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
            'Q' | 'R' | 'B' | 'N' => Ok((
                String::from("P"),
                (None, None),
                (
                    characters.get(0).unwrap().to_string(),
                    characters.get(1).unwrap().to_string(),
                ),
                MoveTypes::Promote(characters.get(2).unwrap().to_string()),
                check_for_check_or_mate(&move_string),
            )),
            '=' => Ok((
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
    use crate::parser::{parse_move, CheckOrCheckMate, MoveTypes};

    // Pawn moves

    #[test]
    fn test_single_pawn_move_forward() {
        let result_move = parse_move("e4").unwrap();
        assert_eq!(
            result_move,
            (
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
            (
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
            (
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
            (
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
            (
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
            (
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
            (
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
            (
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
            (
                String::from("N"),
                (Some("e".to_string()), Some("7".to_string())),
                ("d".to_string(), "3".to_string()),
                MoveTypes::Take,
                CheckOrCheckMate::Neither
            ),
            "A knight taking at d3 from e7"
        );
    }
}
