use rand::Rng;

pub fn generate_eighteen_clues () -> Vec<Vec<u32>> {
    //Board formatted to resemble sudoku
    let mut board: Vec<Vec<u32>> = vec![

        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],

        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],

        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0],
        vec![ 0,0,0,  0,0,0,  0,0,0]
    ];

    //Repeats the process twice 
    for _i in 1..=2 {
        let numbers_to_place: Vec<u32> = (1..=9).collect();

        //Iterates through each of the quads on the board
        for quad in 1..=9 {
            //Stores result of calls to randomly pick an index and number to place
            let cells_in_quad = produce_indexes(quad);
            let (row_pos, col_pos) = pick_index(cells_in_quad, board.clone());
            let index = pick_number(&numbers_to_place);
            let number_choice = numbers_to_place[index];

            //Check whether that number can go there and if it can, remove it from the list

            println!("{} in quad {} in position [{},{}]", number_choice, quad, row_pos, col_pos);
            board[row_pos as usize][col_pos as usize] = number_choice;
        }
    }
    board
}


//Picks a random index from the list provided and returns it
fn pick_number (number_collection: &Vec<u32>) -> usize {
    if number_collection.is_empty() {
        100 //Temporary way of returning a value indicating an error
    }
    else {
        //Generating random index based on current length of vector
        let number_of_options = number_collection.len();
        let index = rand::thread_rng().gen_range(0..number_of_options);
        index
    }
}

//Takes the indexes in a quad
fn pick_index (index_collection: Vec<(u32, u32)>, board: Vec<Vec<u32>>) -> (u32, u32) {
    let number_of_indexes = index_collection.len();
    let index = rand::thread_rng().gen_range(0..number_of_indexes);
    let (rand_index_choice_row, rand_index_choice_col) = index_collection[index];

    if index_collection.is_empty() {
        println!("empty index choices");
        (100,100)
    }
    else if board[rand_index_choice_row as usize][rand_index_choice_col as usize] != 0 {
        let mut modified_collection = index_collection.clone();
        modified_collection.remove(index);
        pick_index(modified_collection, board)
    }
    else {
        (rand_index_choice_row, rand_index_choice_col)
    }

}

//Takes two indexes and returns which quad they are in
fn _determine_quad (row: u32, col: u32) -> u32 {
    if row / 3 == 0 {
        if col / 3 == 0 {1} 
        else if col / 3 == 1 {2}
        else {3}
    }
    else if row / 3 == 1 {
        if col / 3 == 0 {4}
        else if col / 3 == 1 {5}
        else {6}
    }
    else {
        if col / 3 == 0 {7}
        else if col / 3 == 1 {8}
        else {9}
    }
}

//Takes a quad and returns the indexing boundaries of it
fn determine_quad_coords (quad: u32) -> (u32, u32, u32, u32) {
    match quad {
    //         row  col
        1 => {(0,2, 0,2)},
        2 => {(0,2, 3,5)},
        3 => {(0,2, 6,8)},
        4 => {(3,5, 0,2)},
        5 => {(3,5, 3,5)},
        6 => {(3,5, 6,8)},
        7 => {(6,8, 0,2)},
        8 => {(6,8, 3,5)},
        _ => {(6,8, 6,8)},
    }
}

//Takes a quad and returns the 2D indexes of all of the spots inside of it
fn produce_indexes (quad: u32) -> Vec<(u32, u32)> {
    //Getting the bounds to inform the start and end indexes of the ranges 
    let (row_start, row_end, col_start, col_end) = determine_quad_coords(quad);

    let mut indexes: Vec<(u32, u32)> = Vec::new();

    for i in row_start..=row_end {
        for j in col_start..=col_end {
            indexes.push((i,j));
        }
    }

    indexes
}

    /*
    //To be used in test code
    for row in 0..=8 {
        for col in 0..=8 {
            println!("{}", determine_quad(row, col));
        }
    }
    */

    /*
    for i in 1..=9 {
        determine_indexes(i);
    }
    */