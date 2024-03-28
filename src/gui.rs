use crate::setup::utilities::valid;
use crate::setup::utilities::every_spot_full;
use crate::tests::test_cases::valid_board;
use crate::setup::board_generation::generate_eighteen_clues;

use iced::{
    window, Application, Command, Theme, alignment, executor, theme, Length, Settings,
};
use iced::widget::{
    button, Column, Container, Row, Text, TextInput
};

const CELL_SIZE: f32 = 36.0;
const PADDING: f32 = 1.0;

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked(usize, usize),
    TextInputChanged(String),
    MakeNewBoard,
    ResetBoard,
    ExitApp,
}

#[derive(Default)]
struct Grid {
    matrix: Vec<Vec<u32>>,
    awaiting_input: Option<(usize, usize)>,
    clues_positions: Vec<(usize, usize)>,
    finished: bool
}

impl Application for Grid {
    type Executor = executor::Default;
    type Flags = Vec<Vec<u32>>;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Grid {
                clues_positions: find_clues_locations(&flags),
                matrix: flags,
                awaiting_input: None,
                finished: false
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Sudoku")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ButtonClicked(i, j) => {
                if !self.clues_positions.contains(&(i, j)) && !self.finished {
                    // Assigning some value to this opens a text box in view() function
                    self.awaiting_input = Some((i, j));
                }
                Command::none()
            }
    
            Message::TextInputChanged(input) => {
                match self.awaiting_input { // Is there a text box open right now (redundant check)
                    Some((i, j)) => {
                        match input.parse().ok() {
                            Some(val) => {
                                // Place where user types valid number
                                
                                if valid(&self.matrix, val, (i as u32, j as u32)) || val == 0 {
                                    self.matrix[i][j] = val;
                                } else {
                                    println!("Invalid move!");

                                }
                                
                                self.awaiting_input = None;

                                if every_spot_full(&self.matrix) && valid_board(&self.matrix) {
                                    println!("CONGRATS YOU WIN WOOOOO");
                                    self.finished = true;
                                }
                                Command::none()
                            }
                            None => {
                                println!("Invalid value! Make sure you enter a number 0 - 9");
                                self.awaiting_input = None;
                                Command::none()
                            }
                        }
                    }
                    None => Command::none(),
                }
            }

            Message::MakeNewBoard => {
                self.matrix = generate_eighteen_clues();
                self.clues_positions = find_clues_locations(&self.matrix);
                self.awaiting_input = None;
                Command::none()
            }

            Message::ResetBoard => {
                self.matrix = reset_board(&self.matrix, &self.clues_positions);
                Command::none()
            }

            Message::ExitApp => {
                std::process::exit(0);
            }

        }
    }
    

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let matrix = self.matrix.clone();

        let mut sudoku_app = Row::new()
            .padding(PADDING)
            // .spacing(50)
            .align_items(alignment::Alignment::Center);

        let mut base = Column::new()
            .padding(PADDING)
            .spacing(CELL_SIZE/4.0)
            .align_items(alignment::Alignment::Center);

        let title_card = Text::new("Rust Sudoku!")
            .size(40)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);

        base = base.push(title_card);

        let (edit_row, edit_col) = match self.awaiting_input {
            Some((r, c)) => {(r,c)}
            None => {(100, 100)}
        };


        for (i,row)  in matrix.iter().enumerate() {
            let mut gui_row = Row::new().padding(PADDING).spacing(CELL_SIZE/4.0);
    
            for (j,value) in row.iter().enumerate() {
                let num = match value {
                    0 => "".to_string(),
                    _ => {
                        value.to_string()
                    },
                };

                let text_element = Text::new(num)
                    .size(20)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center);
                    
                    
                let button = button(text_element)
                    .width(CELL_SIZE)
                    .height(CELL_SIZE)
                    .on_press(Message::ButtonClicked(i, j))
                    .style(theme::Button::Secondary);
                
                if j % 3 == 0 && j != 0 {
                    gui_row = gui_row.push(Column::new().padding(PADDING));
                }

                if i == edit_row && j == edit_col {

                    let input = TextInput::new(
                        "",
                        "",)
                    .on_input(Message::TextInputChanged)
                    .size(20)
                    .width(CELL_SIZE);           

                    gui_row = gui_row.push(input);
                } else {
                    gui_row = gui_row.push(button);
                }
                
            }
            
            if i % 3 == 0 && i != 0 {
                base = base.push(Row::new().padding(PADDING));
            }

            base = base.push(gui_row);
        }



        let subtitle = Text::new("By Olivia, Kenneth, Jace, and Jorge")
            .size(20)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center);


        base = base.push(subtitle);
        sudoku_app = sudoku_app.push(base);

        let mut side_panel = Column::new()
            .padding(20)
            .spacing(CELL_SIZE/4.0)
            .align_items(alignment::Alignment::Center);

        

        side_panel = side_panel.push(make_button("New Board", Message::MakeNewBoard));
        side_panel = side_panel.push(make_button("Reset Board", Message::ResetBoard));
        side_panel = side_panel.push(make_button("Exit", Message::ExitApp));

        sudoku_app = sudoku_app.push(side_panel);


        Container::new(sudoku_app) 
            .width(Length::Fill) 
            .height(Length::Fill) 
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center) 
            .style(theme::Container::Transparent)
            .into()
    }

}


pub fn launch_gui(initial_board:&Vec<Vec<u32>>) -> iced::Result {
    
    let window_settings = window::Settings {
        size: iced::Size {width: 800.0, height: 600.0},
        ..window::Settings::default()
    };

    let settings = Settings {
        flags: initial_board.clone(),
        window: window_settings,
        ..Settings::default()
    };    

    Grid::run(settings)
}

fn find_clues_locations(initial_board:&Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut locs:Vec<(usize,usize)> = vec![];
    for (i,row) in initial_board.iter().enumerate() {
        for (j,value) in row.iter().enumerate() {
            
            if *value != 0 {
                locs.push((i,j))
            }
            
        }
    }
    locs
}

fn reset_board(initial_board:&Vec<Vec<u32>>, initial_coords:&Vec<(usize, usize)>) -> Vec<Vec<u32>> {
    let mut board = initial_board.clone();
    for (i,row) in initial_board.iter().enumerate() {
        for (j,_) in row.iter().enumerate() {
            if !initial_coords.contains(&(i, j)) {
                board[i][j] = 0;
            }
        }

    }
    board
    
}

fn make_button(text:&str, message:Message) -> iced::widget::Button<'_, Message> {
    let new_board_text = Text::new(text)
        .size(18)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);

    button(new_board_text)
        .width(130)
        .height(50)
        .on_press(message)
        .style(theme::Button::Secondary)

}