mod setup;
mod tests;
mod gui;
use crate::setup::board_generation::generate_eighteen_clues;
use crate::setup::solvability_check::generate_solve_board;
use crate::setup::utilities::{every_spot_full, print_board};
use crate::tests::test_cases::valid_board;

fn main() {
    let mut board_found = false;
    //Goes until it a valid & solvable board has been produced
    while !board_found {
        //Generates the hints
        let mut generated_clues = generate_eighteen_clues();
        //Keeps a copy for displaying before handing it off to the solveable function
        let clues_for_display = generated_clues.clone();
        //Attempts to solve the board based on the clues
        let solved_board = generate_solve_board(&mut generated_clues);
        //If it gives back a fully filled and valid board its done
        if every_spot_full(solved_board) && valid_board(solved_board) {
            println!("Hints: ");
            print_board(&clues_for_display);
            println!("");
            println!("Filled in Board: ");
            print_board(solved_board);

            //Indicates that a board has been found
            board_found = true;
            gui::launch_gui(&clues_for_display);
        }
    }
}
