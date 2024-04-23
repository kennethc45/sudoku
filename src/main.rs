mod setup;
mod tests;
mod gui;
use crate::setup::board_generation::generate_solvable_clues;

fn main() {
    let board_for_display = generate_solvable_clues();
    gui::launch_gui(&board_for_display);
}