use crate::crossword::Crossword;

pub fn generate_crossword_html(crossword: &Crossword) -> String {
    let output = format!(
r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Simple Crossword</title>

<style>
    body {{
        font-family: Arial, sans-serif;
        padding: 20px;
        max-width: 600px;
        margin: auto;
    }}

    #grid {{
        display: grid;
        grid-template-columns: repeat({no_cols}, 40px);
        gap: 4px;
        justify-content: center;
    }}

    .cell {{
        width: 40px;
        height: 40px;
        border: 1px solid #aaa;
        text-align: center;
        font-size: 22px;
    }}

    .cell input {{
        width: 100%;
        height: 100%;
        border: none;
        text-align: center;
        font-size: 22px;
    }}

    .clueBox {{
        width: 40px;
        text-align: center;
        vertical-align: middle;
        font-size: 22px;
        background: #e0e0e0;
    }}

    .black {{
        background: #333;
    }}

    #validateBtn {{
        margin-top: 20px;
        padding: 10px 20px;
        font-size: 18px;
    }}

    .correct {{
        background-color: #cce6ff !important; /* light blue */
    }}

    .incorrect {{
        background-color: #ffcccc !important; /* light red */
    }}

    #clues {{
        margin-top: 30px;
    }}

    @media (max-width: 480px) {{
        #grid {{
            grid-template-columns: repeat({no_rows}, 32px);
        }}
        .cell {{
            width: 32px;
            height: 32px;
        }}
        .cell input {{
            font-size: 18px;
        }}
    }}
</style>
</head>
<body>

<h2>Simple Crossword</h2>

<div id="grid"></div>

<button id="validateBtn">Validate</button>

<div id="clues">
    <h3>Clues</h3>
    <ul id="clueList"></ul>
</div>

<script>
// ------------------------------
// Crossword Data Structure
// ------------------------------
const crossword = {{
{solution_definition},
{clue_definition}
}};

// ------------------------------
// Build the grid
// ------------------------------
const grid = document.getElementById("grid");

for (let r = 0; r < crossword.rows; r++) {{
    for (let c = 0; c < crossword.cols; c++) {{
        const cell = document.createElement("div");
        cell.classList.add("cell");

        let sol_content = crossword.solution[r][c];

        if (sol_content === null) {{
            cell.classList.add("black");
        }}
        else if (isNumber(sol_content)) {{
            const clueBox = document.createElement("clueBox");
            clueBox.textContent = sol_content;
            cell.appendChild(clueBox);
        }} else {{
            const input = document.createElement("input");
            input.maxLength = 1;
            input.dataset.row = r;
            input.dataset.col = c;
            cell.appendChild(input);
        }}

        grid.appendChild(cell);
    }}
}}

// ------------------------------
// Display clues
// ------------------------------
const clueList = document.getElementById("clueList");
crossword.clues.forEach(cl => {{
    const li = document.createElement("li");
    li.textContent = `${{cl.number}}: ${{cl.clue}}`;
    clueList.appendChild(li);
}});

// ------------------------------
// Validation logic
// ------------------------------
document.getElementById("validateBtn").addEventListener("click", () => {{
    const inputs = document.querySelectorAll(".cell input");

    inputs.forEach(input => {{
        const r = input.dataset.row;
        const c = input.dataset.col;
        const correct = crossword.solution[r][c].toUpperCase();

        if (input.value.toUpperCase() === correct) {{
            input.classList.remove("incorrect");
            input.classList.add("correct");
        }} else {{
            input.classList.remove("correct");
            input.classList.add("incorrect");
        }}
    }});
}});

function isNumber(str){{
    const tryNum = Number(str);
    return !isNaN(tryNum);
}}
</script>

</body>
</html>
"#,
    no_cols = crossword.get_no_cols(),
    no_rows = crossword.get_no_rows(),
    solution_definition = generate_solutions_str(crossword),
    clue_definition = generate_clues_str(crossword));

    return output;
}

fn generate_solutions_str(crossword: &Crossword) -> String
{
    /*
    Output expected to look like this:
        rows: 5,
        cols: 5,

        // Use null for black squares
        solution: [
            ["H", "E", "L", "L", "O"],
            ["A", null, "R", null, "E"],
            ["P", "I", "Z", "Z", "A"],
            [null, "N", null, "O", null],
            ["C", "O", "D", "E", "R"]
        ]
    */

    let mut output:String = String::new();

    let min_x = crossword.min_x();
    let max_x = crossword.max_x() + 1;
    let min_y = crossword.min_y();
    let max_y = crossword.max_y() + 1;

    output.push_str(format!("\tcols: {}, \n", crossword.get_no_cols()).as_str());
    output.push_str(format!("\trows: {}, \n", crossword.get_no_rows()).as_str());

    output.push_str("solution: [ \n");

    for y in min_y .. max_y {
        output.push_str("\t\t[");
        
        for x in min_x .. max_x {
            output.push_str(
                match crossword.get_char_at(x, y) {
                    Some('_') => "null, ".to_string(),
                    Some(w) => format!("\"{}\", ", w),
                    None => "null, ".to_string()
                }.as_str());
            }
        output.push_str("],\n");
    }


    output.push_str("\t]");

    return output;
}

fn generate_clues_str(crossword: &Crossword) -> String
{
    /*
    Output expected to look like this:
    clues: [
            {{ number: 1, clue: "Greeting (5 letters)" }},
            {{ number: 2, clue: "Italian food (5 letters)" }},
            {{ number: 3, clue: "Someone who writes software (5 letters)" }}
        ]
    */

    let mut output:String = String::from("clues: [");

    for clue in crossword.get_clues() {
        output.push_str(format!("{{ number: {}, clue: \"{}\" }}, \n", clue.id, clue.text).as_str());
    }

    output.push_str("]");

    return output;
}