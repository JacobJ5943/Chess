use chess::board;
use chess::game::{is_board_check_mate, is_board_draw_by_repetition, is_board_stale_mate};
use chess::parser;
use chess::parser::{MoveTypes, ParsedMove};
use chess::piece_types::PieceColor;
use chess::board::MoveError;

#[test]
fn test_play_game_1() {
    let mut board = board::Board::new();

    //https://lichess.org/rklpc7mk
    let parsed_moves = parser::parse_game_moves(String::from("1. e4 c6 2. Nc3 d5 3. Qf3 dxe4 4. Nxe4 Nd7 5. Bc4 Ngf6 6. Nxf6+ Nxf6 7. Qg3 Bf5 8. d3 Bg6 9. Ne2 e6 10. Bf4 Nh5 11. Qf3 Nxf4 12. Nxf4 Be7 13. Bxe6 fxe6 14. Nxe6 Qa5+ 15. c3 Qe5+ 16. Qe3 Qxe3+ 17. fxe3 Kd7 18. Nf4 Bd6 19. Nxg6 hxg6 20. h3 Bg3+ 21. Kd2 Raf8 22. Rhf1 Ke7 23. d4 Rxf1 24. Rxf1 Rf8 25. Rxf8 Kxf8 26. e4 Ke7 27. Ke3 g5 28. Kf3 Be1 29. Kg4 Bd2 30. Kf5 Bc1 31. Kg6 Kf8 32. e5 Bxb2 33. Kxg5 Bxc3 34. h4 Bxd4 35. h5 Bxe5 36. g4 Bb2 37. Kf5 Kf7 38. g5 Bc1 39. g6+ Ke7 40. Ke5 b5 41. Kd4 Kd6 42. Kc3 c5 43. a3 Bg5 44. a4 bxa4 45. Kb2 Kd5 46. Ka3 Kd4 47. Kxa4 c4"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }

    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "This game did not end in a stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        false,
        "This game did not end in a draw by repetition"
    );
}
#[test]
fn test_play_game_2() {
    let mut board = board::Board::new();

    // https://lichess.org/6x5nq6qd
    let parsed_moves = parser::parse_game_moves(String::from("1. e4 b6 2. Bc4 Bb7 3. d3 Nh6 4. Bxh6 gxh6 5. Qf3 e6 6. Nh3 Bg7 7. c3 Nc6 8. Qg3 Rg8 9. Qf3 Ne5 10. Qe3 Nxc4 11. dxc4 Qe7 12. O-O Qc5 13. Qxc5 b5 14. Qxb5 Bxe4 15. Nd2 Bc6 16. Qb3 Bxc3 17. g3 Bxd2 18. Rad1 Bg5 19. Nxg5 hxg5 20. Qd3 h6 21. b4 Ba4 22. Rd2 Rb8 23. b5 d6 24. Qa3 Bxb5 25. cxb5 Rxb5 26. Qxa7 Rc5 27. Qa8+ Ke7 28. Qxg8 e5 29. Qh8 d5 30. Qxe5+ Kd7 31. Rxd5+ Rxd5 32. Qxd5+ 1-0"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }
    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "This game did not end in a stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        false,
        "This game did not end in a draw by repetition"
    );
}
#[test]
fn test_play_game_3() {
    let mut board = board::Board::new();

    //https://lichess.org/vb3w3rmn
    let parsed_moves = parser::parse_game_moves(String::from("1. e4 c5 2. f4 d5 3. exd5 Qxd5 4. Nc3 Qd8 5. Bc4 Bf5 6. d3 a6 7. g4 Bd7 8. a4 e6 9. Bd2 Bc6 10. Nf3 Bxf3 11. Qxf3 Qh4+ 12. Qg3 Qxg3+ 13. hxg3 Nc6 14. O-O-O O-O-O 15. f5 Ne5 16. fxe6 Nxc4 17. dxc4 fxe6 18. Rde1 Bd6 19. Bf4 Bxf4+ 20. gxf4 Nh6 21. g5 Nf5 22. Rxe6 Rd4 23. Rf1 Rxc4 24. Re5 g6 25. Kd2 Rd8+ 26. Kc1 Rd7 27. Nd5 Rd6 28. Ne7+ Nxe7 29. Rxe7 Rd7 30. Rxd7 Kxd7 31. b3 Re4 32. Kb2 Ke6 33. Kc3 Kf5 34. Rh1 Re7 35. Rf1 Re4 36. Rh1 Rxf4 37. Rxh7 Kxg5 38. Rxb7 Rf6 39. Rc7 Kf4 40. Rxc5 g5 41. b4 g4 42. Rc4+ Kf3 43. Rc5 Rg6 44. Rf5+ Kg2 45. b5 axb5 46. axb5 g3 47. Kb4 Kh1 48. Rd5 g2 49. Rd1+ g1=Q 50. Rxg1+ Kxg1 51. c4 Kf2 52. c5 Ke3 53. b6 Kd4 54. b7 Rg1 55. Kb5 Rb1+ 56. Kc6 Rb4 57. Kc7 Kxc5 58. b8=Q Rxb8 59. Kxb8 1/2-1/2"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }
    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "This game did not end in a stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        false,
        "This game did not end in a draw by repetition"
    );
}
#[test]
fn test_play_game_4() {
    let mut board = board::Board::new();

    //https://lichess.org/v778e8mr
    let parsed_moves = parser::parse_game_moves(String::from("1. d4 d5 2. c4 dxc4 3. e4 g6 4. Bxc4 Bg7 5. Ne2 Nf6 6. Nbc3 O-O 7. O-O e6 8. Be3 Nbd7 9. f3 Nb6 10. Bd3 a5 11. b3 c6 12. Kh1 Nbd7 13. Qd2 b5 14. a4 b4 15. Na2 Ba6 16. Nac1 Bxd3 17. Nxd3 Qc7 18. Bf4 Qb6 19. Rac1 Nh5 20. Bh6 Bxh6 21. Qxh6 Ng7 22. Rc4 e5 23. dxe5 Rfe8 24. f4 Qe3 25. Nec1 Nb6 26. Re1 Nxc4 27. Rxe3 Nxe3 28. Qh3 Nd1 29. Qf3 Nc3 30. f5 gxf5 31. exf5 Rad8 32. h3 c5 33. f6 Ne6 34. Nxc5 Kh8 35. Nc1d3 Rg8 36. Nxe6 fxe6 37. Nf4 Rde8 38. f7 Rgf8 39. fxe8=Q Rxe8 40. Qg4 Na2 41. Nxe6 Rg8 42. Qf5 h6 43. Qf6+ Kh7 44. Nf8+ Rxf8 45. Qxf8 Nc1 46. Qf7+ Kh8 47. e6 1-0
"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }
    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "This game did not end in a stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        false,
        "This game did not end in a draw by repetition"
    );
}
#[test]
fn test_play_game_5() {
    let mut board = board::Board::new();

    //https://lichess.org/tgrspcpg
    let parsed_moves = parser::parse_game_moves(String::from("1. e4 e5 2. Nf3 d6 3. Bc4 Nd7 4. d3 h6 5. c3 Ngf6 6. O-O Be7 7. h3 O-O 8. Be3 Kh8 9. Nbd2 Nh7 10. Re1 Ndf6 11. Nf1 Nh5 12. Nxe5 dxe5 13. Qxh5 Nf6 14. Qxe5 Bd6 15. Qb5 c6 16. Qb3 Qe7 17. a4 Qe5 18. Bd4 Qg5 19. e5 Bxh3 20. Ng3 Ng4 21. gxh3 Nxf2 22. Kxf2 Qd2+ 23. Re2 Qf4+ 24. Kg2 Bc7 25. Rf1 Qg5 26. Rf5 Qg6 27. Re4 f6 28. Rg4 Qe8 29. exf6 Bxg3 30. fxg7+ Kh7 31. gxf8=N+ Qxf8 32. Rxf8 Rxf8 33. Rg7+ 1-0"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }
    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "This game did not end in a stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        false,
        "This game did not end in a draw by repetition"
    );
}

#[test]
fn test_en_passant_white() {
    let mut board = board::Board::new();
    let parsed_moves = parser::parse_game_moves(String::from("1. c4 h6 2. c5 b5 3. cxb6 h5"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }
}

#[test]
fn test_en_passant_white_one_move_late() {
    let mut board = board::Board::new();
    let parsed_moves = parser::parse_game_moves(String::from("1. c4 b5 2. c5 e6"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }
    let white_move = parser::parse_move("cxb6").unwrap();
    match white_move.move_type {
        MoveTypes::FinalResult(_game_result) => assert!(false, "The game should not have ended"),
        _ => {
            let result = board.play_move(white_move);
            if let Ok(_) = result {
                assert!(
                    false,
                    "Expected en passant to be one move late and fail, but it succeeded. Move cxb6"
                );
            };
        } // White move
    };
}

#[test]
fn test_en_passant_black() {
    let mut board = board::Board::new();
    let parsed_moves = parser::parse_game_moves(String::from("1. h3 d5 2. h4 d4 3. e4 dxe3"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }
}

#[test]
fn test_en_passant_black_one_move_late() {
    let mut board = board::Board::new();
    let parsed_moves = parser::parse_game_moves(String::from("1. h3 d5 2. e4 d4"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        count = count + 1;
    }

    let white_move = parser::parse_move("h4").unwrap();
    match white_move.move_type {
        MoveTypes::FinalResult(_game_result) => assert!(false, "The game should not have ended"),
        _ => {
            let result = board.play_move(white_move);
            if let Err(error) = result {
                assert!(
                    false,
                    "Move before en passant failed unexpectedly.Error:{:?}",
                    error
                );
            }
        } // White move
    };
    let black_move = parser::parse_move("dxe3").unwrap();
    match black_move.move_type {
        MoveTypes::FinalResult(_game_result) => assert!(false, "The game should not have ended"),
        _ => {
            let result = board.play_move(black_move);
            if let Ok(_) = result {
                assert!(
                    false,
                    "Expected en passant to be one move late and fail, but it succeeded. Move dxe3"
                );
            };
        }
    };
}

#[test]
fn test_stalemate_game_1() {
    // https://lichess.org/c4s8ru73

    let mut board = board::Board::new();

    let parsed_moves = parser::parse_game_moves(String::from("1. d4 d5 2. c4 Nf6 3. Nf3 Bg4 4. Nbd2 Bxf3 5. Nxf3 e6 6. g3 Bb4+ 7. Bd2 a5 8. Bg2 dxc4 9. Qa4+ Nc6 10. Bxb4 axb4 11. Qb5 O-O 12. O-O Nxd4 13. Qxb4 Nxe2+ 14. Kh1 Nd5 15. Qxc4 Nxg3+ 16. fxg3 Qd6 17. Rad1 f5 18. Nd4 Ra5 19. Nxe6 Qxe6 20. Bxd5 Rxd5 21. Qxd5 Qxd5+ 22. Rxd5 g5 23. Rd7 f4 24. Rxc7 f3 25. Rxb7 g4 26. a4 Re8 27. b4 Re2 28. a5 f2 29. Kg2 h5 30. Rxf2 Re8 31. Ra2 Ra8 32. a6 Kf8 33. a7 h4 34. Rb8+ Rxb8 35. axb8=Q+ Kf7 36. Ra7+ Ke6 37. Qf4 Kd5 38. Ra6 h3+ 39. Kf2 1/2-1/2"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // Black move
        }
        count = count + 1;
    }

    assert_eq!(
        is_board_stale_mate(&mut board),
        true,
        "Expected the board to be in stalemate"
    );
}

#[test]
fn test_stalemate_game_2() {
    //https://lichess.org/tg9m322f

    let mut board = board::Board::new();

    let parsed_moves = parser::parse_game_moves(String::from("1. e4 d5 2. exd5 Nf6 3. Nc3 c6 4. dxc6 Nxc6 5. Nf3 Bg4 6. Be2 e5 7. h3 Bh5 8. g4 Bg6 9. g5 Ne4 10. Nxe4 Bxe4 11. h4 Nd4 12. Rh3 Nxc2+ 13. Kf1 Qd7 14. Rg3 Nxa1 15. d3 Bc6 16. Nxe5 Qd5 17. d4 Qh1+ 18. Rg1 Bg2+ 19. Ke1 Qxg1+ 20. Kd2 Bb4+ 21. Kd3 Be4+ 22. Kxe4 O-O 23. Qxg1 Nc2 24. Be3 Rad8 25. Bd3 Nxe3 26. fxe3 Rfe8 27. Kf3 Bd6 28. Ng4 Re7 29. Qc1 Rde8 30. Qc2 h5 31. Bh7+ Kh8 32. Ne5 Bxe5 33. dxe5 Rxe5 34. e4 g6 35. Qc7 Re5e7 36. Qc4 Kxh7 37. Qd5 a6 38. b3 b5 39. a3 Kg8 40. Qc6 Rxe4 41. Qxa6 Re3+ 42. Kf2 Rxb3 43. a4 bxa4 44. Qxa4 Rb2+ 45. Kf3 Rh2 46. Qxe8+ Kg7 47. Qe5+ Kf8 48. Qh8+ Ke7 49. Qf6+ Kf8 50. Qd8+ Kg7 51. Qf6+ Kf8 52. Qd8+ Kg7 53. Qf6+ Kf8 54. Qd6+ Kg7 55. Qxh2 Kf8 56. Qb8+ Kg7 57. Qe8 Kh7 58. Qxf7+ Kh8 59. Qxg6 1/2-1/2"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // Black move
        }
        count = count + 1;
    }

    assert_eq!(
        is_board_stale_mate(&mut board),
        true,
        "Expected the board to be in stalemate"
    );
}

// more stalemate
/*
https://lichess.org/hbnzj1yf

 */

#[test]
fn test_draw_by_repetition_1() {
    // https://lichess.org/72f4jdy3
    let mut board = board::Board::new();

    let parsed_moves = parser::parse_game_moves(String::from("1. e4 b6 2. d4 Bb7 3. Bd3 Nf6 4. Qe2 e6 5. c4 Be7 6. Nc3 O-O 7. h4 h6 8. Bf4 d6 9. Qd2 Ng4 10. Nf3 Nd7 11. Rh3 e5 12. Bxh6 Nxh6 13. Rg3 Bxh4 14. Qxh6 Bxg3 15. Qh3 Bf4 16. Nh4 g6 17. Ke2 Kg7 18. Rh1 Rh8 19. g3 Bg5 20. dxe5 Nxe5 21. f4 Nxd3 22. Kxd3 Bxh4 23. gxh4 Qf6 24. Ne2 Qxb2 25. f5 Rh6 26. fxg6 fxg6 27. h5 Rah8 28. Qd7+ Kg8 29. Qe8+ Kg7 30. Qe7+ Kg8 31. Qe8+ Kg7 32. Qe7+ Kg8 33. Qe8+ Kg7 34. Qe7+ Kg8 1/2-1/2"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // Black move
        }
        count = count + 1;
    }

    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "Expected the board to be in stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        true,
        "Expected the board to be in draw by repetition"
    );
}

#[test]
fn test_draw_by_repetition_2() {
    //https://lichess.org/0ub1zd6g
    let mut board = board::Board::new();

    let parsed_moves = parser::parse_game_moves(String::from("1. e4 g6 2. d4 Bg7 3. e5 e6 4. Nc3 d6 5. f4 dxe5 6. fxe5 Ne7 7. Nf3 O-O 8. Bc4 b6 9. Bg5 Bb7 10. O-O c5 11. dxc5 Qxd1 12. Raxd1 bxc5 13. Na4 Nbc6 14. Nxc5 Rab8 15. Nxb7 Rxb7 16. b3 Nf5 17. a3 h6 18. Bc1 Nxe5 19. Nxe5 Bxe5 20. Rfe1 Bd4+ 21. Kh1 Bc3 22. Bd2 Bb2 23. Bc1 Bc3 24. Bd2 Bb2 25. Bc1 Bc3 1/2-1/2"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // Black move
        }
        count = count + 1;
    }

    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "Expected the board to be in stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        true,
        "Expected the board to be in draw by repetition"
    );
}

#[test]
fn more_check_mate_tests_1() {
    //https://lichess.org/l6uc9nf2
    let mut board = board::Board::new();

    let parsed_moves = parser::parse_game_moves(String::from("1. e4 g6 2. d4 Bg7 3. Nf3 e6 4. c4 c6 5. Bd3 b6 6. O-O Ne7 7. Nc3 d5 8. cxd5 cxd5 9. e5 Nbc6 10. Bg5 Qd7 11. Nb5 O-O 12. Nd6 Nxd4 13. Bxe7 Qxe7 14. Nxc8 Nxf3+ 15. Qxf3 Raxc8 16. Qg3 f6 17. exf6 Bxf6 18. Rab1 Bg7 19. h4 e5 20. h5 g5 21. Ba6 Rc7 22. Qd3 Rd8 23. Rfc1 Rxc1+ 24. Rxc1 e4 25. Qb3 Bh6 26. Rc8 g4 27. Qxd5+ Kg7 28. Qxd8 Qb4 29. Rc7+ Qe7 30. Qxe7+ Kg8 31. Rc8+ Bf8 32. Qxf8# 1-0"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // Black move
        }
        count = count + 1;
    }

    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "Expected the board to be in stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        false,
        "Expected the board to be in draw by repetition"
    );
    assert_eq!(
        is_board_check_mate(&board.last_move_color.clone(), &mut board),
        true,
        "Expected the board to be in checkmate"
    );
}

#[test]
fn more_check_mate_tests_2() {
    //https://lichess.org/ovkepufy
    let mut board = board::Board::new();

    let parsed_moves = parser::parse_game_moves(String::from("1. d4 d5 2. Nc3 Nc6 3. Bf4 Bf5 4. Nb5 Rc8 5. a3 f6 6. e3 e5 7. dxe5 fxe5 8. Bg3 Qg5 9. h4 Qg4 10. Be2 Qg6 11. h5 Qf6 12. Bh4 g5 13. hxg6 Qxg6 14. Bh5 Qxh5 15. Qxh5+ Bg6 16. Qd1 Rd8 17. Nxc7+ Kf7 18. Bxd8 Nge7 19. Bxe7 Bxe7 20. Qxd5+ Kg7 21. O-O-O Nd8 22. Ne6+ Nxe6 23. Qxe6 h5 24. Rd7 Kh6 25. Rxe7 Rc8 26. Qxe5 Rxc2+ 27. Kd1 Rxb2 28. Qxb2 a5 29. Nf3 a4 30. Qf6 b5 31. Qg5# 1-0"));
    let mut count = 0;
    for parsed_move in parsed_moves {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();
        match white_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(white_move);
                //println!("QuickBoard:{}\n{:?}", count, board.position_board);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::WHITE,
                        true,
                        "The expected last move color was white, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // White move
        }
        match black_move.move_type {
            MoveTypes::FinalResult(_game_result) => (),
            _ => {
                let result = board.play_move(black_move);
                //println!("QuickBoard:{}\n{:?}", count, board.position_board);
                match result {
                    Ok(_) => assert_eq!(
                        &board.last_move_color == &PieceColor::BLACK,
                        true,
                        "The expected last move color was black, but got {:?}",
                        &board.last_move_color
                    ),
                    Err(move_error) => assert!(false, "Error:{:?}", move_error),
                }
            } // Black move
        }
        count = count + 1;
    }

    assert_eq!(
        is_board_stale_mate(&mut board),
        false,
        "Expected the board to be in stalemate"
    );
    assert_eq!(
        is_board_draw_by_repetition(&mut board),
        false,
        "Expected the board to be in draw by repetition"
    );
    assert_eq!(
        is_board_check_mate(&board.last_move_color.clone(), &mut board),
        true,
        "Expected the board to be in checkmate"
    );
}

#[test]
fn test_fools_mate() {
    let mut board = board::Board::new();

    let parsed_moves = parser::parse_game_moves(String::from("1. f3 e5 1. g4 Qh4#"));


    for ((white_coords, black_coords), parsed_move) in [(((5,2), (5,1)),
                         ((4,4),(4,6))),
        (((6,3), (6,1)),
         ((7,3), (4,0)))].iter().zip(parsed_moves) {
        let white_move = parsed_move.1.unwrap();
        let black_move = parsed_move.2.unwrap();

        match board.play_move(white_move) {
            Ok(_) => {
                if let None = board.find_piece_color(white_coords.0.0, white_coords.0.1, &PieceColor::WHITE) {
                    assert!(false, "There was not a white piece on {:?}", white_coords.0.1);
                }

                if let Some(_) = board.find_piece_color(white_coords.1.0, white_coords.1.1, &PieceColor::WHITE) {
                    assert!(false, "The wrong piece moved to {:?} from {:?}", white_coords.0, white_coords.1)
                }
            }
            Err(error) => {assert!(false, "There was an error playing white move:{:?}", error)}
        }

        match board.play_move(black_move) {
            Ok(_) => {
                if let None = board.find_piece_color(black_coords.0.0, black_coords.0.1, &PieceColor::BLACK) {
                    assert!(false, "There was not a black piece on {:?}", black_coords.0.1);
                }

                if let Some(_) = board.find_piece_color(black_coords.1.0, black_coords.1.1, &PieceColor::BLACK) {
                    assert!(false, "The wrong piece moved to {:?} from {:?}", black_coords.0, black_coords.1)
                }
            }
            Err(error) => {assert!(false, "There was an error playing black move:{:?}", error)}
        }

    }

}
