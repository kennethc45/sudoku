use iced::application::Appearance;
use iced::{
    color, executor, theme, widget, Background, Color, Command, Element, Length, Point, Rectangle, Sandbox, Settings, Size
};
use iced::widget::{Column, Row, Text, Container, container};

const GRID_SIZE: usize = 9;
const CELL_SIZE: f32 = 30.0;
const PADDING: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
enum Message {
    Number(Option<u32>),
    
}

#[derive(Default)]
struct Grid {
    matrix: [[i32; GRID_SIZE]; GRID_SIZE],
}


impl Sandbox for Grid {
    type Message = Message;

    fn new() -> Self {
        Self {
            matrix: get_board()
        }
    }

    fn title(&self) -> String {
        String::from("PLEASE HELP ME")
    }

    fn update(&mut self, message:Message) {
        match message {
            Message::Number(value) => {
                // Handle number input here
                println!("Received number: {:?}", value);
            }
        }
    }
    // Old view function that only displayed text w/ no interaction
// fn view(&self) -> iced::Element<'_, Self::Message> {
    //     let matrix = self.matrix;
    //     let mut base = Column::new().padding(PADDING).spacing(CELL_SIZE);
        
    //     for row in matrix {
    //         let mut gui_row = Row::new().padding(PADDING).spacing(CELL_SIZE);
            
    //         for &value in &row {
    //             let text = value.to_string();
    //             let text_element = Text::new(text).size(20);
    //             gui_row = gui_row.push(text_element);
    //         }
            
    //         base = base.push(gui_row);
    //     }
        
    //     base.into()
    
    // }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let matrix = self.matrix;
        let mut base = Column::new().padding(PADDING).spacing(CELL_SIZE/4.0);
    
        for row in matrix {
            let mut gui_row = Row::new().padding(PADDING).spacing(CELL_SIZE/4.0);
    
            for value in row {
                let text = match value {
                    0 => "".to_string(),
                    _ => value.to_string(),
                };
                let text_element = Text::new(text)
                    .size(20);
                
                let container = Container::new(text_element)
                    .width(CELL_SIZE)
                    .height(CELL_SIZE)
                    .center_x()
                    .center_y()
                    .style(theme::Container::Box);
                    
                gui_row = gui_row.push(container);
            }
    
            base = base.push(gui_row);
        }
    
        base.into()
    }

}


pub fn launch_gui() -> iced::Result {
    Grid::run(Settings::default())
}

fn get_board() -> [[i32; GRID_SIZE]; GRID_SIZE] {
    // Temporary board, will eventually pull from actual one
    [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ]
}
