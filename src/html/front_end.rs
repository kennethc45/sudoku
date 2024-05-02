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




