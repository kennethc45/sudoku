mod setup;
mod tests;
use crate::tests::testCases::main2;
use crate::setup::board_generation::generate_twenty_clues;
use crate::setup::utilities::print_board;

fn main() {
    let generated_board = generate_twenty_clues();
    print_board(&generated_board);
    main2();
}