mod setup;
mod gui;
use crate::setup::board_generation::generate_twenty_clues;
use crate::setup::utilities::print_board;

fn main() {
    let generated_board = generate_twenty_clues();
    gui::launch_gui(&generated_board);
    print_board(&generated_board);
}