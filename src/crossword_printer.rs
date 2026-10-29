use crate::crossword::Crossword;
use crate::crossword::CellValue;
use crate::crossword::ClueDirection;
use crate::crossword::ClueStartCell;

pub fn print(crossword: &Crossword) {
    
    println!("Created crossword");
    println!("");
    println!("");

    let min_x = crossword.min_x();
    let max_x = crossword.max_x() + 1;
    let min_y = crossword.min_y();
    let max_y = crossword.max_y() + 1;

    for y in min_y .. max_y {
        for x in min_x .. max_x {
            match crossword.get_cellvalue_at(x, y) {
                Some(CellValue::ClueEnd()) => {print!(" ")}
                Some(CellValue::Normal(w)) => print!("{}", w),
                Some(CellValue::ClueStart(w)) => match w {
                        ClueStartCell::Horizontal(ww)
                        | ClueStartCell::Vertical(ww)
                        | ClueStartCell::HorizontalVertical(ww) => print!("{}", ww),
                    }
                None => print!(" ")
            };
        }
        println!("");
    }

    println!("");
    println!("");

    println!("Horizontal clues:");

    for clue in crossword.get_clues().filter(|c| c.direction == ClueDirection::Horizontal) {
        println!("{}: {}", clue.id, clue.text);
    }

    println!("");

    println!("Vertical clues:");

    for clue in crossword.get_clues().filter(|c| c.direction == ClueDirection::Vertical) {
        println!("{}: {}", clue.id, clue.text);
    }
}