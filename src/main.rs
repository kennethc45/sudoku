mod setup;
mod tests;
mod gui;
use crate::setup::board_generation::generate_eighteen_clues;
use crate::setup::solvability_check::generate_solve_board;
use crate::setup::utilities::{every_spot_full, print_board,valid_board};

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
            /*
            let complete_valid_board: Vec<Vec<u32>> = vec![
                // 0's represent empty spaces
    
                vec![6,3,9, 5,7,4, 1,8,2],
                vec![5,4,1, 8,2,9, 3,7,6],
                vec![7,8,2, 6,1,3, 9,5,4],
    
                vec![1,9,8, 4,6,7, 5,2,3],
                vec![3,6,5, 9,8,0, 4,1,7],
                vec![4,2,7, 1,3,5, 8,6,9],
    
                vec![9,5,6, 7,4,8, 2,3,1],
                vec![8,1,3, 2,9,6, 7,4,5],
                vec![2,7,4, 3,0,1, 6,9,8]
            ];
            */
            // tokio::spawn(async move {
            //     gui::launch_gui(&clues_for_display);
            // });
            gui::launch_gui(&clues_for_display);
        }
    }

    let server_address = std::env::var("SERVER_ADRESS")
        .unwrap_or("127.0.0.1:3000".to_owned());

    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/post", post(handle_input));

    axum::serve(listener, app)
        .await
        .expect("Error serving application")


}


#[derive(Debug, Serialize, Deserialize)]
struct Coordinates {
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Input {
    coordinates: Coordinates,
    value: i32,
}

async fn handle_input(data: axum::extract::Json<Input>) -> String {
    // Process the received JSON data
    let bounds = if data.coordinates.x < 0 || data.coordinates.x > 8 {
        "X coordinates must be within 0-8".to_string()
    } else if data.coordinates.y < 0 || data.coordinates.y > 8 {
        "Y coordinates must be within 0-8".to_string()
    } else if data.value < 1 || data.value > 9 {
        "Values can only be between 1-9".to_string()
    } else {
        "Valid Input".to_string()
    };

    format!("Received coordinates: ({}, {}), Value: {}, {}",
            data.coordinates.x, 
            data.coordinates.y, 
            data.value,
            bounds
           )
}