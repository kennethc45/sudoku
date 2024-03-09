
mod setup;
use crate::setup::board_generation::generate_eighteen_clues;
use crate::setup::utilities::print_board;
use crate::setup::utilities::valid;

fn main() {
    let board: Vec<Vec<u32>> = vec![
    // 0's represent empty spaces

    vec![1,0,0, 0,0,0, 0,0,0],
    vec![0,0,0, 0,0,0, 0,0,0],
    vec![0,0,0, 0,0,0, 0,0,0],

    vec![0,0,0, 0,0,0, 0,0,0],
    vec![0,0,0, 0,0,0, 0,0,0],
    vec![0,0,0, 0,0,0, 0,0,0],

    vec![0,0,0, 0,0,0, 0,0,0],
    vec![0,0,0, 0,0,0, 0,0,0],
    vec![0,0,0, 0,0,0, 0,0,0]
    ];

    /*
    print_board(&board);
    valid(&board, 1, (1,2));    
    valid(&board, 9, (1,1)); // True
    valid(&board, 1, (0,1));
    valid(&board, 1, (1,0));
    valid(&board, 1, (4,4)); // True
    */

    let generated_board = generate_eighteen_clues();
    print_board(&generated_board);
     
}




