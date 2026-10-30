use crate::crossword::Crossword;
use crate::crossword::CellValue;
use crate::crossword::ClueDirection;
use crate::crossword::ClueStartCell;
use crate::Entry;
use crate::config::Config;

use chrono;

pub fn generate_crossword_html(crossword: &Crossword, ai_gen_clues: &Vec<Entry>, config: &Config) -> String {
    let date_str = chrono::Local::now().date_naive().checked_add_days(chrono::Days::new(1)).expect("REASON").format("%A, %Y-%m-%d").to_string();
    
    let output = format!(
r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Daily SE Mini {date}</title>

    <style>
    :root {{
        --grid-size: min(90vw, 520px);
        --cell-size: calc(var(--grid-size) / 10);

        --radius: 12px;

        --blue: #2563eb;
        --blue-dark: #1d4ed8;
        --blue-light: #dbeafe;

        --border: #cbd5e1;
        --text: #1e293b;
        --text-secondary: #64748b;

        --success: #0cff0c1a;
        --error: #fee2e2;
    }}

    body {{
        font-family:
            Inter,
            system-ui,
            -apple-system,
            BlinkMacSystemFont,
            sans-serif;

        background: #f8fafc;
        color: var(--text);

        max-width: 900px;
        margin: 0 auto;
        padding: 24px 16px;

        line-height: 1.5;
    }}

    h2 {{
        text-align: center;
        margin-bottom: 24px;
        font-size: clamp(1.5rem, 5vw, 2rem);
    }}

    h3 {{
        margin-top: 1.5rem;
    }}

    /* Crossword */

    #grid {{
        --cell-size: calc(min(90vw, 520px) / var(--cols));

        display: grid;
        grid-template-columns: repeat(var(--cols), var(--cell-size));

        width: max-content;
        margin: 0 auto;

        padding: 6px;
        gap: 2px;

        background: var(--border);
        border-radius: var(--radius);
    }}

    .cell {{
        aspect-ratio: 1;

        background: white;
        border: 1px solid var(--border);

        display: flex;
        align-items: center;
        justify-content: center;

        position: relative;
        box-sizing: border-box;
    }}

    .cell input {{
        width: 100%;
        height: 100%;

        border: none;
        outline: none;

        background: transparent;

        text-align: center;
        text-transform: uppercase;

        font-size: clamp(16px, 4vw, 26px);
        font-weight: 600;
    }}

    .clue-cell {{
        background: var(--blue-light);
        color: #1e3a8a;

        font-size: clamp(14px, 3vw, 22px);
        font-weight: 700;
    }}

    .transparent {{
        background: none;
        border: none;
    }}

    .cell input.correct {{
        background: var(--success);
    }}

    .cell input.incorrect {{
        background: var(--error);
    }}

    /* Clues */

    #Ledtrådar {{
        margin-top: 32px;
        padding: 20px;

        background: white;
        border-radius: var(--radius);

        box-shadow: 0 1px 3px rgba(0, 0, 0, .08);
    }}

    #Ledtrådar ul {{
        padding-left: 20px;
    }}

    #Ledtrådar li {{
        margin-bottom: 8px;
    }}

    /* Buttons */

    #validateBtn,
    .toggle-button,
    .submit-button {{
        border: none;
        cursor: pointer;

        transition:
            background .2s ease,
            transform .15s ease;
    }}

    #validateBtn:hover,
    .toggle-button:hover,
    .submit-button:hover {{
        transform: translateY(-1px);
    }}

    #validateBtn:active,
    .toggle-button:active,
    .submit-button:active {{
        transform: translateY(0);
    }}

    #validateBtn {{
        display: block;

        margin: 24px auto;
        padding: 12px 24px;

        background: var(--blue);
        color: white;

        border-radius: 10px;

        font-size: 16px;
        font-weight: 600;
    }}

    .toggle-button,
    .submit-button {{
        background: var(--blue);
        color: white;
    }}

    .toggle-button:hover,
    .submit-button:hover {{
        background: var(--blue-dark);
    }}

    .submit-button {{
        flex-shrink: 0;

        padding: 8px 14px;

        border-radius: 6px;
        font-weight: 600;
    }}

    .submit-button:disabled {{
        background: #94a3b8;
        cursor: not-allowed;
    }}

    /* Accessibility */

    input:focus,
    button:focus-visible {{
        outline: 3px solid #60a5fa;
        outline-offset: 2px;
    }}

    /* AI Section */

    .toolbar {{
        margin-bottom: 12px;
    }}

    .component-list {{
        display: flex;
        flex-direction: column;
        gap: 12px;

        overflow: hidden;

        max-height: 1000px;
        opacity: 1;

        transition:
            max-height .3s ease,
            opacity .3s ease;
    }}

    .component-list.collapsed {{
        max-height: 0;
        opacity: 0;
    }}

    .submit-component {{
        display: flex;
        flex-wrap: wrap;
        align-items: flex-start;
        gap: 12px;

        padding: 14px;

        background: white;

        border: 1px solid #e2e8f0;
        border-radius: var(--radius);

        box-shadow: 0 1px 2px rgba(0, 0, 0, .05);
    }}

    .submit-content {{
        flex: 1 1 250px;

        overflow-wrap: anywhere;
        word-break: break-word;
    }}

    .secondary {{
        color: var(--text-secondary);
    }}
    </style>
</head>
<body>

<h2>Daily SE Mini {date}</h2>

<div id="grid"></div>

<button id="validateBtn">Validate</button>

<div id="Ledtrådar">
    <h3>Vågrätt</h3>
    <ul id="horizontalClueList"></ul>
    <h3>Lodrätt</h3>
    <ul id="verticalClueList"></ul>
</div>

<div class="toolbar">
    <button id="toggleButton" class="toggle-button">
        Show AI Generated Clues
    </button>
</div>

<div id="aiGenClueList" class="component-list collapsed"></div>

<script>
// ------------------------------
// Crossword Data Structure
// ------------------------------
    const crossword = {{
        {solution_definition},
        {clue_definition},
        {ai_gen_clues}
    }};

    let isAIGenSectionExpanded = false;

// ------------------------------
// Build the grid
// ------------------------------
const grid = document.getElementById("grid");
grid.style.setProperty("--cols", {no_cols});

for (let r = 0; r < crossword.rows; r++) {{
    for (let c = 0; c < crossword.cols; c++) {{
        const cell = document.createElement("div");
        cell.classList.add("cell");

        let sol_content = crossword.solution[r][c];

        if (sol_content === null) {{
            cell.classList.add("transparent");
        }}
        else if (isNumber(sol_content)) {{
            cell.classList.add("clue-cell");
            cell.textContent = sol_content;
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
const horizontalClueList = document.getElementById("horizontalClueList");
crossword.horizontalClues.forEach(cl => {{
    const li = document.createElement("li");
    li.textContent = `${{cl.number}}: ${{cl.clue}}`;
    horizontalClueList.appendChild(li);
}});

const verticalClueList = document.getElementById("verticalClueList");
crossword.verticalClues.forEach(cl => {{
    const li = document.createElement("li");
    li.textContent = `${{cl.number}}: ${{cl.clue}}`;
    verticalClueList.appendChild(li);
}});

// ------------------------------
// AI Generated Clues section
// ------------------------------

    const container = document.getElementById("aiGenClueList");
    const toggleButton = document.getElementById("toggleButton");

    function renderItems() {{
        container.innerHTML = "";

        crossword.ai_gen_clues.forEach(item => {{
            const component = document.createElement("div");

            component.className = "submit-component";

            component.innerHTML = `
                <div class="submit-content">
                    <b>${{item.key}}:</b>
                    ${{item.clue}}
                    <span class="secondary">(${{item.confidence}})</span>
                </div>

                <button
                    class="submit-button"
                    type="submit">
                    Godkänn
                </button>
            `;

            const button = component.querySelector(".submit-button");

            button.addEventListener("click", () => {{
                submitClueToDB(button, item);
            }});

            container.appendChild(component);
        }});
    }}

    function updateExpandedState() {{
        container.classList.toggle("collapsed", !isAIGenSectionExpanded);

        toggleButton.textContent =
            isAIGenSectionExpanded ? "Stäng": "Visa AI genererade ledtrådar";
    }}

    toggleButton.addEventListener("click", () => {{
        isAIGenSectionExpanded = !isAIGenSectionExpanded;
        updateExpandedState();
    }});

    async function submitClueToDB(button, item) {{
        try {{
            button.disabled = true;
            button.textContent = "...";

            const response = await fetch("{db_url}", {{
                method: "POST",
                headers: {{
                    "Content-Type": "application/json"
                }},
                body: JSON.stringify({{
                    key: item.key,
                    clue: item.clue,
                    aI_gen_confidence: item.confidence
                }})
            }});

            if (!response.ok) {{
                throw new Error("Request failed");
            }}

            button.textContent = "Inskickad";
        }}
        catch (error) {{
            console.error(error);

            button.textContent = "Err";
        }}
    }}

    renderItems();
    updateExpandedState();

// ------------------------------
// Validation logic
// ------------------------------
document.getElementById("validateBtn").addEventListener("click", () => {{
    const inputs = document.querySelectorAll(".cell input");

    inputs.forEach(input => {{
        const r = input.dataset.row;
        const c = input.dataset.col;
        const correct = crossword.solution[r][c].toUpperCase();

        if (input.value != "") {{
            if (input.value.toUpperCase() === correct) {{
                input.classList.remove("incorrect");
                input.classList.add("correct");
            }} else {{
                input.classList.remove("correct");
                input.classList.add("incorrect");
            }}
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
    date = date_str,
    no_cols = crossword.get_no_cols(),
    solution_definition = generate_solutions_str(crossword),
    clue_definition = generate_clues_str(crossword),
    ai_gen_clues = generate_ai_clues(ai_gen_clues),
    db_url = &config.clue_get_api);

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
                match crossword.get_cellvalue_at(x, y) {
                    Some(CellValue::ClueEnd()) => "null, ".to_string(),
                    Some(CellValue::Normal(w)) => format!("\"{}\", ", w),
                    Some(CellValue::ClueStart(w)) => match w {
                        ClueStartCell::Horizontal(ww)
                        | ClueStartCell::Vertical(ww)
                        | ClueStartCell::HorizontalVertical(ww) => format!("\"{}\", ", ww),
                    }
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
    horizontalClues: [
            {{ number: 1, clue: "Greeting (5 letters)" }},
            {{ number: 2, clue: "Italian food (5 letters)" }},
            {{ number: 3, clue: "Someone who writes software (5 letters)" }}
        ];
    verticalClues: [
            {{ number: 1, clue: "Greeting (5 letters)" }},
            {{ number: 2, clue: "Italian food (5 letters)" }},
            {{ number: 3, clue: "Someone who writes software (5 letters)" }}
        ]
    */

    let mut output:String = String::from("horizontalClues: [\n");

    let mut sorted_clues: Vec<_> = crossword.get_clues().collect();
    sorted_clues.sort_by_key(|w| w.id);

    for clue in sorted_clues.iter().filter(|w| w.direction == ClueDirection::Horizontal) {
        output.push_str(format!("{{ number: {}, clue: \"{}\" }}, \n", clue.id, clue.text).as_str());
    }

    output.push_str("],  \nverticalClues: [\n");

    for clue in sorted_clues.iter().filter(|w| w.direction == ClueDirection::Vertical) {
        output.push_str(format!("{{ number: {}, clue: \"{}\" }}, \n", clue.id, clue.text).as_str());
    }

    output.push_str("]");

    return output;
}

fn generate_ai_clues(clues:&Vec<Entry>) -> String
{
    /*
    Output expected to look like this:
        ai_gen_clues = [
            {{
                key: "key1",
                clue: "clue1",
                confidence: 80,
            }},
            {{
                key: "key2",
                clue: "clue2",
                confidence: 20,
            }},
            {{
                key: "key3",
                clue: "clue3",
                confidence: 50,
            }},
        ];
    */

    let mut output:String = String::from("ai_gen_clues: [\n");

    for clue in clues {
        output.push_str(format!("{{ key: \"{}\", clue: \"{}\", confidence: \"{}\" }}, \n", clue.get_key(), clue.get_clue(), clue.get_confidence()).as_str());
    }

    output.push_str("]");

    return output;
}