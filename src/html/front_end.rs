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

pub fn new_board() -> &'static str {
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




