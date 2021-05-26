use chess::board;
use chess::parser;
use chess::parser::MoveTypes;
use chess::piece_types::PieceColor;

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
