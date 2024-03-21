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
    println!("");
}

fn valid(board: &Vec<Vec<u32>>, val: u32, pos: (u32, u32)) -> bool {
    // Column - pos[0]; Row - pos[1]
    // Checks if there is a repeat of the value in the position's column
    for i in 0..board.len() {
        if (board[pos.0 as usize][i] == val) && (pos.1 != i.try_into().unwrap()) {
            println!("Invalid board");
            println!("repeat in row: {}", i);
            return false;
        }
    }

    // Checks if there is a repeat of the value in the position's row
    for i in 0..board[0].len() {
        if (board[i][pos.1 as usize] == val) && (pos.0 != i.try_into().unwrap()) {
            println!("Invalid board");
            println!("repeat in column: {}", i);
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
                println!("Invalid board");
                println!("repeat in a box:{}",  box_x);
                return false;
            }
        }
    }
    return true;
}

fn vaildBoard(board: &Vec<Vec<u32>>) -> bool{
    for a in 0..board.len() {
       for b in 0..board.len() {
          let result = valid(&board, board[a][b],(a as u32, b as u32));
          if (result == false) {
            return false;
          }
       }
    }
    println!("Board is vaild");
    return true;
}

fn updateBoard(board: &mut Vec<Vec<u32>>, val: u32, pos: (u32, u32)) -> &Vec<Vec<u32>> {
    board[pos.0 as usize][pos.1 as usize] = val;
    
    let result = valid(&board,board[pos.0 as usize][pos.1 as usize],(pos.0 as u32,pos.1 as u32));
    if result == false{
        println!("Move is not vaild,Try again");
    }
    else{
        println!("Move is vaild");
    }
    return board;
}

fn main() {
    let hintsOnlyBoard: Vec<Vec<u32>> = vec![
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

    let incompleteBoard: Vec<Vec<u32>> = vec![
        // 0's represent empty spaces
    
        vec![5,6,8, 3,0,9, 4,0,2],
        vec![0,0,2, 0,0,6, 0,0,0],
        vec![0,0,7, 4,0,2, 6,3,0],
    
        vec![6,8,5, 0,0,4, 1,0,0],
        vec![2,0,9, 0,3,0, 7,0,6],
        vec![7,0,0, 0,6,1, 0,0,9],
    
        vec![9,0,6, 5,0,3, 0,1,7],
        vec![0,0,0, 6,0,7, 0,0,0],
        vec![0,7,3, 1,9,0, 2,6,0]
    ];
    
    let completeValidBoard: Vec<Vec<u32>> = vec![
        // 0's represent empty spaces
    
        vec![6,3,9, 5,7,4, 1,8,2],
        vec![5,4,1, 8,2,9, 3,7,6],
        vec![7,8,2, 6,1,3, 9,5,4],
    
        vec![1,9,8, 4,6,7, 5,2,3],
        vec![3,6,5, 9,8,2, 4,1,7],
        vec![4,2,7, 1,3,5, 8,6,9],
    
        vec![9,5,6, 7,4,8, 2,3,1],
        vec![8,1,3, 2,9,6, 7,4,5],
        vec![2,7,4, 3,5,1, 6,9,8]
    ];

    let repeatColumnBoard: Vec<Vec<u32>> = vec![
        // 0's represent empty spaces
    
        vec![6,3,9, 5,7,4, 1,8,2],
        vec![6,4,1, 8,2,9, 3,7,6],
        vec![7,8,2, 6,1,3, 9,5,4],
    
        vec![1,9,8, 4,6,7, 5,2,3],
        vec![3,6,5, 9,8,2, 4,1,7],
        vec![4,2,7, 1,3,5, 8,6,9],
    
        vec![9,5,6, 7,4,8, 2,3,1],
        vec![8,1,3, 2,9,6, 7,4,5],
        vec![2,7,4, 3,5,1, 6,9,8]
    ];

    let repeatRowBoard: Vec<Vec<u32>> = vec![
        // 0's represent empty spaces
    
        vec![6,3,3, 5,7,4, 1,8,2],
        vec![5,4,1, 8,2,9, 3,7,6],
        vec![7,8,2, 6,1,3, 9,5,4],
    
        vec![1,9,8, 4,6,7, 5,2,3],
        vec![3,6,5, 9,8,2, 4,1,7],
        vec![4,2,7, 1,3,5, 8,6,9],
    
        vec![9,5,6, 7,4,8, 2,3,1],
        vec![8,1,3, 2,9,6, 7,4,5],
        vec![2,7,4, 3,5,1, 6,9,8]
    ];

    let mut test_Moveboard: Vec<Vec<u32>> = vec![
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

    // print_board(&test_Moveboard);
    // valid(&test_Moveboard, 1, (1,2));    
    // valid(&test_Moveboard, 9, (1,1)); // True
    // valid(&test_Moveboard, 1, (0,1));
    // valid(&test_Moveboard, 1, (1,0));
    // valid(&test_Moveboard, 1, (4,4)); // True

    // vaildBoard(&incompleteBoard);
    // vaildBoard(&completeValidBoard);
    // vaildBoard(&repeatColumnBoard);
    // vaildBoard(&repeatRowBoard);

    print_board(&test_Moveboard);
    updateBoard(&mut test_Moveboard, 1, (0, 1));
    print_board(&test_Moveboard);




}



