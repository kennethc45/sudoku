mod setup;
mod tests;
mod html;

use crate::setup::board_generation::generate_solvable_clues;
use crate::setup::utilities::valid_board;
use crate::html::front_end::new_board;

use axum::response::{IntoResponse, Html};
use axum::Json;
use axum::{routing::{get, post}, Router};

use minijinja::{Environment, Template, render};

use serde::{Deserialize, Serialize};
use setup::utilities::print_board;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let address_num = "127.0.0.1:3000";

    let server_address = std::env::var("SERVER_ADRESS")
        .unwrap_or(address_num.to_owned());

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    // Defined endpoints 
    let app = Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/new_game/", get(|| handle_new_board(address_num)))
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

async fn handle_new_board(base_address: &str) -> impl IntoResponse {
    let current_board: Vec<Vec<u32>> = generate_solvable_clues();
    print_board(&current_board.clone());
    Html(render!(new_board(), board => current_board, location => base_address))
}