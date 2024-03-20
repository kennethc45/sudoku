mod setup;
mod gui;
use crate::setup::board_generation::generate_twenty_clues;
use crate::setup::utilities::print_board;

fn main() {
    gui::launch_gui();
    let generated_board = generate_twenty_clues();
    print_board(&generated_board);
}