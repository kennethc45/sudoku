mod setup;
mod tests;
mod html;

use std::cmp::Ordering;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::setup::board_generation::generate_solvable_clues;
use crate::setup::utilities::valid_board;
use crate::html::front_end::new_board;

use axum::response::{IntoResponse, Html};
use axum::Json;
use axum::{routing::{get, post}, Router, extract::State};

use minijinja::render;

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    play_boards: Vec<Vec<Vec<u32>>>,
    current_board: Arc<Mutex<usize>>
}

#[tokio::main]
async fn main() {
    //Creating the boards and updating the state to hold them and the current board the user is on
    let mut boards: Vec<Vec<Vec<u32>>> = Vec::new();
    for _ in 0..100 {
        boards.push(generate_solvable_clues());
    }

    let state = AppState {
        play_boards: boards,
        current_board: Arc::new(Mutex::new(0))
    };


    let server_address = std::env::var("SERVER_ADRESS")
        .unwrap_or("127.0.0.1:3000".to_owned());

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    // Defined endpoints 
    let app = Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/new_game", get(handle_new_board))
        .route("/spot_check", post(spot_check))
        .route("/win_check", post(win_check))
        .with_state(state);

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

//Displays to the user a html page with a new board
async fn handle_new_board(State(state): State<AppState>) -> impl IntoResponse {
    let mut current_board:MutexGuard<usize> = state.current_board.lock().expect("Modifying current board.");
    
    //Handles incrementing to the next board and wrapping around when the user goes through all of them
    if current_board.cmp(&99) == Ordering::Equal{
        *current_board = 0;
    }
    else {
        *current_board = *current_board + 1;
    }

    //Looking up the current board and calling the method that will return the html for rendering it
    let current_board: Vec<Vec<u32>> = state.play_boards.get(*current_board).unwrap().to_vec();
    Html(render!(new_board(), board => current_board))
}