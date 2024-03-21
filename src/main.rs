mod setup;
mod gui;
use crate::setup::board_generation::generate_twenty_clues;
use crate::setup::utilities::print_board;
use crate::gui::launch_gui;

fn main() {
    let generated_board = generate_twenty_clues();
    print_board(&generated_board);
    launch_gui(&generated_board); // Should probably be the last thing called in main
}