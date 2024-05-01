mod setup;
mod tests;
mod gui;
use crate::setup::board_generation::generate_eighteen_clues;
use crate::setup::solvability_check::generate_solve_board;
use crate::setup::utilities::{every_spot_full, valid_board, print_board};

use axum::response::IntoResponse;
use axum::Json;
use axum::{
    routing::{get, post}, Router,
};

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
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

            // gui::launch_gui(&clues_for_display);
        }
    }

    let server_address = std::env::var("SERVER_ADRESS")
        .unwrap_or("127.0.0.1:3000".to_owned());

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    // Defined endpoints 
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/spot_check", post(spot_check))
        .route("/win_check", post(win_check));

    // Launches the local server
    axum::serve(listener, app)
        .await
        .expect("Error serving application")


}


#[derive(Debug, Serialize, Deserialize)]
struct Coordinates {
    x: u32,
    y: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct SudokuBoard {
    board: Vec<Vec<u32>>
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    coordinates: Coordinates,
    value: u32,
    board: SudokuBoard
}

// Reads the users input and checks if it is valid within board
async fn spot_check(data: axum::extract::Json<Input>) -> impl IntoResponse{
    use crate::setup::utilities::valid;

    let Input {
        coordinates: Coordinates {x, y}, 
        value,
        board: SudokuBoard { board }
    } = data.0;

    // Keep for reference for postman testing
    // let mut board: Vec<Vec<u32>> = vec![
    
    //             vec![0,0,2, 0,0,0, 0,3,0],
    //             vec![0,4,0, 3,0,0, 6,0,0],
    //             vec![0,0,0, 0,2,0, 0,0,0],
    
    //             vec![0,0,4, 0,0,0, 0,0,5],
    //             vec![0,0,0, 0,8,0, 0,0,7],
    //             vec![8,0,0, 0,1,0, 0,0,0],
    
    //             vec![0,0,0, 0,0,0, 0,5,9],
    //             vec![0,9,0, 0,0,1, 0,0,0],
    //             vec![0,0,7, 0,6,0, 0,0,0]
    //         ];

    
            let is_valid = valid(&board, value, (x, y));

            Json(is_valid)
}

// Checks if the board is a valid solution.
async fn win_check(sudoku_board: axum::extract::Json<SudokuBoard>) -> impl IntoResponse {

    let SudokuBoard { board } = sudoku_board.0;

    // If you change one of the values in a valid board to zero it counts as true
    let is_win = valid_board(&board);

    Json(is_win)
}