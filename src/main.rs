fn print_board(board: &Vec<Vec<u32>>) {

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

fn valid(board: &Vec<Vec<u32>>, val: u32, pos: (u32, u32)) -> bool {
    // Column - pos[0]; Row - pos[1]
    // Checks if there is a repeat of the value in the position's column
    for i in 0..board.len() {
        if (board[pos.0 as usize][i] == val) && (pos.1 != i.try_into().unwrap()) {
            println!("False1");
            return false;
        }
    }

    // Checks if there is a repeat of the value in the position's row
    for i in 0..board[0].len() {
        if (board[i][pos.1 as usize] == val) && (pos.0 != i.try_into().unwrap()) {
            println!("False2");
            return false;
        }
    }

    // Checks if there is a repeat of the value in the position's box
    // The boxes should either be 1, 2, or 3
    let box_x = pos.0 / 3;
    let box_y = pos.1 / 3;
 
    for i in (box_x * 3)..(box_x * 3 + 3) {
        for j in (box_y * 3)..(box_y * 3 + 3) {
            if (board[i as usize][j as usize] == val) && ((i, j) != pos) {
                println!("False3");
                return false;
            }
        }
    }
    println!("True");
    return true;
}

fn main() {
    let easyBoard1 : Vec<Vec<u32>> = vec![
    // 0's represent empty spaces

    vec![5,6,8, 3,0,9, 4,0,2],
    vec![0,0,2, 0,0,6, 0,0,0],
    vec![0,0,7, 4,0,2, 6,3,0],

    vec![0,8,5, 0,0,4, 1,0,0],
    vec![2,0,9, 0,3,0, 7,0,6],
    vec![7,0,0, 0,6,1, 0,0,9],

    vec![9,0,0, 5,0,3, 0,1,7],
    vec![0,0,0, 0,0,7, 0,0,0],
    vec![0,0,3, 1,9,0, 2,6,0]
    ];

    let complete_easyBoard1 : Vec<Vec<u32>> = vec![
        // 0's represent empty spaces
    
        vec![5,6,8, 3,0,9, 4,0,2],
        vec![0,0,2, 0,0,6, 0,0,0],
        vec![0,0,7, 4,0,2, 6,3,0],
    
        vec![0,8,5, 0,0,4, 1,0,0],
        vec![2,0,9, 0,3,0, 7,0,6],
        vec![7,0,0, 0,6,1, 0,0,9],
    
        vec![9,0,0, 5,0,3, 0,1,7],
        vec![0,0,0, 0,0,7, 0,0,0],
        vec![0,0,3, 1,9,0, 2,6,0]
        ];
    
    let test_Moveboard: Vec<Vec<u32>> = vec![
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

    print_board(&test_Moveboard);
    valid(&test_Moveboard, 1, (1,2));    
    valid(&test_Moveboard, 9, (1,1)); // True
    valid(&test_Moveboard, 1, (0,1));
    valid(&test_Moveboard, 1, (1,0));
    valid(&test_Moveboard, 1, (4,4)); // True
     
}



