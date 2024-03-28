use crate::setup::utilities::valid;
use crate::setup::utilities::every_spot_full;
use crate::tests::test_cases::valid_board;
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
                    self.awaiting_input = Some((i, j));
                }
                Command::none()
            }
    
            Message::TextInputChanged(input) => {
                match self.awaiting_input { // Is there a text box open rn 
                    Some((i, j)) => {
                        match input.parse().ok() {
                            Some(val) => {
                                // Place where user types valid number
                                // Should do validity checks here probably
                                if valid(&self.matrix, val, (i as u32, j as u32)) || val == 0 {
                                    self.matrix[i][j] = val;
                                } else {
                                    println!("Invalid move!")
                                }
                                
                                self.awaiting_input = None;

                                if every_spot_full(&self.matrix) && valid_board(&self.matrix) {
                                    println!("CONGRATS YOU WIN WOOOOO");
                                    self.finished = true;
                                }
                                Command::none()
                            }
                            None => {
                                println!("Invalid new value!");
                                self.awaiting_input = None;
                                Command::none()
                            }
                        }
                    }
                    None => Command::none(),
                }
            }
        }
    }
    

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let matrix = self.matrix.clone();

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

        Container::new(base) 
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