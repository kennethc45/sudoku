pub fn start_page() -> &'static str {
    return r#"
        <!doctype html>
        <title> Sudoku </title>
        <h1> Sudoku </h1>
        <h2> Pick A Difficulty Level </h2>
        <form>
            <button type="submit" formaction="http://127.0.0.1:3000/new_game/1"> Easy </button>
            <button type="submit" formaction="http://127.0.0.1:3000/new_game/2"> Medium </button>
            <button type="submit" formaction="http://127.0.0.1:3000/new_game/3"> Hard </button>
        <form>
    "#;
}

pub fn new_board(board: &Vec<Vec<u32>>) -> &'static str {
    let board_json = serde_json::to_string(board).expect("Failed to serailzie board");
    return r#"
        <!doctype html>
        <head>
            <style>
                td {
                    padding: 10px;
                    border-spacing: 20px;
                    border: 1px solid black;
                    height: 20px;
                    width: 20px;
                    text-align: center;
                }
            </style>
        </head>
        <title> Sudoku </title>
        <h1> Sudoku </h1>
        <table>
            {% for row in range(1,10) %}
                <tr>
                {% for col in range(1,10) %}
                    {% if board[row-1][col-1] == 0 %}
                        <td> {{" "}} </td>
                    {% else %}
                        <td>{{board[row-1][col-1]}}</td>
                    {% endif %}
                {% endfor %}
                </tr>
            {% endfor %}
        </table>
        <p></p>
        <h3> Enter a coordinate and a value </h3>
        <label for="x_coordinate">X_Coordinate:</label>
        <input type="text" id="x_coordinate">
        <label for="y_coordinate">Y_Coordinate:</label>
        <input type="text" id="y_coordinate">
        <label for="enter_value">Value:</label>
        <input type="text" id="enter_value">
        <button onclick="updateBoard()"> Update Board </button>
        <script>
                async function updateBoard() {
                    const x_coord = document.getElementById("x_coordinate").value;
                    const y_coord = document.getElementById("y_coordinate").value;
                    const value = document.getElementById("enter_value").value;
                    const board = get_board();
                    const requestData = {
                        coordinates: {x: parseInt(x_coord), y: parseInt(y_coord)},
                        value: parseInt(value),
                        board: board
                    };
                    try {
                        const response = await fetch("http://127.0.0.1:3000/spot_check", {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json'
                            },
                            body: JSON.stringify(requestData)
                        });

                        if (!response.ok) {
                            throw new Error('Network response was not ok');
                        }
                    } catch (error) {
                        console.error('Problem with fetch: ', error);
                    }
 
                }

                function get_board() {
                    var board =[];
                    var tables = document.querySelector('table');
                    var rows = tables.querySelectorAll('tr');

                    rows.forEach(function(row, rowIndex) {
                        var rowData = [];
                        var cells = row.querySelectorAll('td');

                        cells.forEach(function(cell, colIndex) {
                            var cellValue = cell.innerText.trim();
                            rowData.push(parseInt(cellValue) || 0);
                        });
                        board.push(rowData);
                    });
                    return board;
                }
        </script>

        <h3> Request a Hint </h3>
        <label for="requested_row"> Row: </label>
        <input type="text" id="requested_row"> 
        <label for="requested_col"> Column: </label>
        <input type="text" id="requested_col">
        <button onclick="submitCoords()"> Request Hint </button>
        <p id="hint_display_area"> </p>
        <script>
            async function submitCoords() {
                const row = document.getElementById("requested_row").value;
                const col = document.getElementById("requested_col").value;
                
                if (isNaN(row) || isNaN(col)) {
                    document.getElementById("hint_display_area").innerHTML = "Please only enter numbers.";
                }
                else {
                    if (row > 9 || row < 1 || col > 9 || col < 1) {
                        document.getElementById("hint_display_area").innerHTML = "Please enter coordinates between 1 and 9.";
                    }
                    else {
                        const url = "http://127.0.0.1:3000/get_hint/" + row + "/" + col + "";
                        fetch(url).then(result => result.json()).then(value => {
                            if (value == 0){
                                document.getElementById("hint_display_area").innerHTML = "Error retrieving the requested hint."
                            }
                            else {
                                document.getElementById("hint_display_area").innerHTML = "[" + row + "] [" + col + "] is " + value + ".";
                            }
                        });
                    }
                }
            }
        </script>
        <p></p>
        <form>
            {% if difficulty == 1 %}
                <button type="submit" formaction="http://127.0.0.1:3000/new_game/1"> New Board </button>
            {% elif difficulty == 2 %}
                <button type="submit" formaction="http://127.0.0.1:3000/new_game/2"> New Board </button>
            {% else %}
                <button type="submit" formaction="http://127.0.0.1:3000/new_game/3"> New Board </button>
            {% endif %}
        </form>
        <p></p>
        <form action="http://127.0.0.1:3000/">
            <button type="submit"> Return to Home </button>
        </form>
        <p></p>
        <form action="http://127.0.0.1:3000/solution">
            <button type="submit"> Show Solution </button>
        </form>
    "#;
}

pub fn solution_board() -> &'static str {
    return r#"
        <!doctype html>
        <head>
            <style>
                td {
                    padding: 10px;
                    border-spacing: 20px;
                    border: 1px solid black;
                    height: 20px;
                    width: 20px;
                    text-align: center;
                }
            </style>
        </head>
        <title> Sudoku </title>
        <h1> Sudoku </h1>
        <table>
            {% for row in range(1,10) %}
                <tr>
                {% for col in range(1,10) %}
                    <td>{{board[row-1][col-1]}}</td>
                {% endfor %}
                </tr>
            {% endfor %}
        </table>
        <p></p>
        <button onclick="goBack()">
            Return to game 
        </button>
        <script>
            function goBack() {
                window.history.back();
            }
        </script>
    "#;
}




