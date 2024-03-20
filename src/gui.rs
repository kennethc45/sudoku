use iced::alignment::Vertical;
// use iced::application::Appearance;
use iced::{
    window,alignment, color, executor, theme, widget, Background, Color, Command, ContentFit, Element, Length, Point, Rectangle, Sandbox, Settings, Size
};
use iced::widget::{TextInput, button, container, Button, Column, Container, Row, Text, text, column};

const GRID_SIZE: usize = 9;
const CELL_SIZE: f32 = 36.0;
const PADDING: f32 = 1.0;

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked(usize, usize), // Represents a button click with row and column indices
    // TextInputChanged(String, usize, usize),
    TextInputChanged(String),
}



#[derive(Default)]
struct Grid {
    matrix: Vec<Vec<u32>>,
    awaiting_input: Option<(usize, usize)>,
}


impl Sandbox for Grid {
    type Message = Message;
    // type Flags = Vec<Vec<u32>>;

    fn new() -> Self {
        Self {
            matrix: get_board(),
            awaiting_input: None
        }
    }

    fn title(&self) -> String {
        String::from("PLEASE HELP ME")
    }

    fn update(&mut self, message:Message) {
        match message {
            Message::ButtonClicked(i, j) => {

                self.awaiting_input = Some((i, j));
            
            }
            Message::TextInputChanged(input) => {
                let (i,j) = self.awaiting_input.unwrap();
                match parse_input(input) {
                    Some(val) => {
                        self.matrix[i][j] = val;
                        self.awaiting_input = None;
                    }
                    None => {self.awaiting_input = None;}
                }
                
            }
            _ => {println!("Something horrible has happened");}

        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let matrix = self.matrix.clone();
        let mut base = Column::new().padding(PADDING).spacing(CELL_SIZE/4.0);
        // let (mut editRow, mut editCol) = (-1, -1);

        let (edit_row, edit_col) = match self.awaiting_input {
            Some((row, col)) => {(row,col)}
            None => {(100, 100)}
        };


        for (i,row) in matrix.iter().enumerate() {
            let mut gui_row = Row::new().padding(PADDING).spacing(CELL_SIZE/4.0);
    
            for (j,value) in row.iter().enumerate() {
                let num = match value {
                    0 => "".to_string(),
                    _ => value.to_string(),
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
    
        base.into()
    }

}


pub fn launch_gui(initial_board:&Vec<Vec<u32>>) -> iced::Result {
    // let settings = Settings {
    //     flags: initial_board,
    //     ..Settings::default()
    // };
    // let grid = Grid { matrix: *initial_board, awaiting_input: None };
    // let settings = Settings::with_flags(initial_board);
    // let settings: Settings<Vec<Vec<u32>>> = Settings::with_flags(initial_board.clone());

    Grid::run(Settings::default())
}

fn get_board() -> Vec<Vec<u32>> {
    // Temporary board, will eventually pull from actual one
    vec![
        vec![9, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]
}

fn parse_input(input:String) -> Option<u32> {
    // Parse the character into an integer
    let parsed = input.chars().next()?.to_digit(10)?;

    // Check if the parsed integer is positive and less than or equal to 9
    if parsed > 0 && parsed <= 9 {
        Some(parsed)
    } else {
        None
    }
}