use iced::image::viewer::Renderer;
use iced::{
    button, text_input, Align, Button, Column, Container, Element, Image, Length, Row, Sandbox,
    Settings, Text, TextInput,
};

use crate::board::Board;
use crate::game;
use crate::piece_types::PieceColor;
use crate::pieces::PieceMove;
use iced::button::State;
use std::borrow::Borrow;
use std::collections::HashMap;
use Message::{DecrementPressed, IncrementPressed, InputChanged, PlayMove};

#[derive(Default)]
pub struct GuiRunner {
    game_continue: bool,
    wrong_move_string: String,
    text_state: text_input::State,
    text_value: String,
    board: Board,
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

impl GuiRunner {
    pub fn create_container_from_board<'a>(board: &Board) -> Row<Message> {
        let mut file_name_hash_map = HashMap::new();
        for piece in &board.live_white_pieces {
            let (x, y) = piece.get_pos();
            //println!("FILENAME:{}", game::file_name_from_piece(&piece, &PieceColor::WHITE));
            file_name_hash_map.insert(
                (x, y),
                game::file_name_from_piece(&piece, &PieceColor::WHITE),
            );
        }
        for piece in &board.live_black_pieces {
            let (x, y) = piece.get_pos();
            //println!("FILENAME:{}", game::file_name_from_piece(&piece, &PieceColor::BLACK));
            file_name_hash_map.insert(
                (x, y),
                game::file_name_from_piece(&piece, &PieceColor::BLACK),
            );
        }

        for x in 0..8 {
            for y in 0..8 {
                if !file_name_hash_map.contains_key(&(x, y)) {
                    file_name_hash_map.insert((x, y), String::from("Empty.png"));
                }
            }
        }

        let row = Row::new()
            // Start of column
            .push(
                Column::new()
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(0, 0)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(0, 1)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(0, 2)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(0, 3)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(0, 4)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(0, 5)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(0, 6)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(0, 7)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    ),
            )
            //End of Column
            // Start of column
            .push(
                Column::new()
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(1, 0)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(1, 1)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(1, 2)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(1, 3)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(1, 4)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(1, 5)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(1, 6)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(1, 7)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    ),
            )
            //End of Column
            // Start of column
            .push(
                Column::new()
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(2, 0)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(2, 1)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(2, 2)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(2, 3)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(2, 4)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(2, 5)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(2, 6)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(2, 7)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    ),
            )
            //End of Column
            // Start of column
            .push(
                Column::new()
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(3, 0)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(3, 1)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(3, 2)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(3, 3)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(3, 4)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(3, 5)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(3, 6)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(3, 7)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    ),
            )
            //End of Column
            // Start of column
            .push(
                Column::new()
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(4, 0)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(4, 1)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(4, 2)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(4, 3)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(4, 4)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(4, 5)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(4, 6)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(4, 7)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    ),
            )
            //End of Column
            // Start of column
            .push(
                Column::new()
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(5, 0)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(5, 1)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(5, 2)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(5, 3)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(5, 4)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(5, 5)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(5, 6)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(5, 7)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    ),
            )
            //End of Column
            // Start of column
            .push(
                Column::new()
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(6, 0)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(6, 1)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(6, 2)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(6, 3)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(6, 4)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(6, 5)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(6, 6)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(6, 7)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    ),
            )
            //End of Column
            // Start of column
            .push(
                Column::new()
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(7, 0)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(7, 1)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(7, 2)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(7, 3)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(7, 4)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(7, 5)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(7, 6)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    )
                    .push(
                        Image::new(format!(
                            "{}/resources/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            file_name_hash_map.get(&(7, 7)).unwrap()
                        ))
                        .height(Length::from(100))
                        .height(Length::from(100)),
                    ),
            );
        //End of Column
        row
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    InputChanged(String),
    PlayMove,
}

impl Sandbox for GuiRunner {
    type Message = Message;

    fn new() -> Self {
        let mut return_self = Self::default();
        return_self.game_continue = true;
        return_self
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::InputChanged(string_value) => {
                //self.text_value = string_value;
                if let GuiRunner { text_value, .. } = self {
                    *text_value = string_value;
                }
                //println!("we typing:{}", debug_value) ;
            }
            Message::PlayMove => {
                println!("This is working");
                println!("CONT:{}", self.game_continue);
                if !self.game_continue {
                    return;
                }
                let string_value = &self.text_value;
                // First is this a valid move.  If it's not then I want to update the second line
                // saying it's invalid

                // I will need to add a draw response
                if string_value == &"draw".to_string() {
                    // Do draw things
                } else {
                    match game::player_move(&mut self.board, &string_value) {
                        Ok(_) => {
                            if game::is_board_check_mate(
                                &self.board.last_move_color.clone(),
                                &mut self.board,
                            ) {
                                self.game_continue = false;
                                self.wrong_move_string =
                                    format!("{:?} Wins with checkmate", self.board.last_move_color);
                            // This is where we would change to the next screen if I had one.
                            } else {
                                self.text_value = "".to_string();
                                self.wrong_move_string == "".to_string();
                            }
                        } // Do valid move things,
                        Err(error) => self.wrong_move_string = error.to_string(),
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut row = GuiRunner::create_container_from_board(&self.board);
        let text_input = TextInput::new(
            &mut self.text_state,
            "Type something to continue...",
            &mut self.text_value,
            Message::InputChanged,
        );

        let container = Container::new(row).width(Length::Fill).height(Length::Fill);

        let mut col = Column::new()
            .push(container)
            .push(Text::new(format!(
                "CurrentMoveColor:{:?}",
                PieceColor::opposite_color(&self.board.last_move_color)
            )))
            .push(Text::new(if self.wrong_move_string != "".to_string() {
                &self.wrong_move_string
            } else {
                ""
            }))
            .push(
                TextInput::new(
                    &mut self.text_state,
                    "This is a palceholder",
                    &mut self.text_value,
                    InputChanged,
                )
                .on_submit(PlayMove),
            );
        //.push(Row::new().push(text_input).push(Button::new(&mut self.increment_button, IncrementPressed)));
        col.into()
    }
}
