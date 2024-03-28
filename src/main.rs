mod setup;
mod tests;
mod gui;
use crate::setup::board_generation::generate_eighteen_clues;
use crate::setup::solvability_check::generate_solve_board;
use crate::setup::utilities::{every_spot_full, print_board,valid_board};

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
            // let complete_valid_board: Vec<Vec<u32>> = vec![
            //     // 0's represent empty spaces
            
            //     vec![6,3,9, 5,7,4, 1,8,2],
            //     vec![5,4,1, 8,2,9, 3,7,6],
            //     vec![7,8,2, 6,1,3, 9,5,4],
            
            //     vec![1,9,8, 4,6,7, 5,2,3],
            //     vec![3,6,5, 9,8,0, 4,1,7],
            //     vec![4,2,7, 1,3,5, 8,6,9],
            
            //     vec![9,5,6, 7,4,8, 2,3,1],
            //     vec![8,1,3, 2,9,6, 7,4,5],
            //     vec![2,7,4, 3,5,1, 6,9,8]
            // ];
            gui::launch_gui(&clues_for_display);
        }
    }
}