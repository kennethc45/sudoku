pub fn new_board() -> &'static str {
    return r#"
        <!doctype html>
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

    "#;
}

pub fn solution_board() -> &'static str {
    return r#"
        <!doctype html>
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

    "#;
}


