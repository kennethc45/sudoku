mod setup;
mod tests;
//use crate::tests::testCases::main2::hints_Only_Board;
use crate::setup::board_generation::generate_twenty_clues;
use crate::setup::board_generation::generate_solve_board;
use crate::setup::utilities::print_board;

// [First][Second]
// First number is row; Second number is column

fn main() {
    // let generated_board = generate_twenty_clues();
    // print_board(&generated_board);
    let mut generated_board2 = generate_twenty_clues();
    print_board(&generated_board2);
    println!("");
    println!("----------------------------");
    println!("");
    print_board(generate_solve_board(&mut generated_board2));

}