use iced::alignment::Vertical;
// use iced::application::Appearance;
use iced::{
    alignment, color, executor, theme, widget, Background, Color, Command, ContentFit, Element, Length, Point, Rectangle, Sandbox, Settings, Size
};
use iced::widget::{button, container, Button, Column, Container, Row, Text, text, column};

const GRID_SIZE: usize = 9;
const CELL_SIZE: f32 = 36.0;
const PADDING: f32 = 1.0;

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked(usize, usize), // Represents a button click with row and column indices
    TextInputChanged(String, usize, usize), // Represents text input change with row and column indices
}



#[derive(Default)]
struct Grid {
    // matrix: [[u32; GRID_SIZE]; GRID_SIZE],
    matrix: Vec<Vec<u32>>,

    // tile: TileState
}


impl Sandbox for Grid {
    type Message = Message;

    fn new() -> Self {
        // Grid::default()
        Self {
            matrix: get_board()
        }
    }

    fn title(&self) -> String {
        String::from("PLEASE HELP ME")
    }

    fn update(&mut self, message:Message) {

    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let matrix = self.matrix.clone();
        let mut base = Column::new().padding(PADDING).spacing(CELL_SIZE/4.0);
    
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

                gui_row = gui_row.push(button);
            }
            
            if i % 3 == 0 && i != 0 {
                base = base.push(Row::new().padding(PADDING));
            }

            base = base.push(gui_row);
        }
    
        base.into()
    }

}


pub fn launch_gui() -> iced::Result {
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
