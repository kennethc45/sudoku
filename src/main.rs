fn print_board() {
    let board: Vec<Vec<u32>> = vec![
    // 0's represent empty spaces

    vec![0,0,0,0,0,0,0,0,0],
    vec![0,0,0,0,0,0,0,0,0],
    vec![0,0,0,0,0,0,0,0,0],
    vec![0,0,0,0,0,0,0,0,0],
    vec![0,0,0,0,0,0,0,0,0],
    vec![0,0,0,0,0,0,0,0,0],
    vec![0,0,0,0,0,0,0,0,0],
    vec![0,0,0,0,0,0,0,0,0],
    vec![0,0,0,0,0,0,0,0,0]
];

    for i in 0..board.len() {
        // Separates boxes horizontally
        if (i != 0) && (i % 3 == 0) {
            println!("- - - - - - - - - - - - -");
        }

        for j in 0..board[0].len() {
            // Separates boxes vertically
            if (j != 0) && (j % 3 == 0) {
                print!(" | ");
            }

            if j == 8 {
                // Prints values for last column 
                println!("{}", board[i][j]);
            } else {
                // Prints all other values w/ a space
                // Lines values up with the horizontal dash lines 
                print!("{} ", board[i][j].to_string());
            }
        }
    }
}

fn main() {
    print_board();
}



