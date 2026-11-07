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
                Some(CellValue::ClueStart(w)) => print!("{}", crossword.get_clues().find(|c| c.id == w.get_id()).unwrap().display_number),
                None => print!(" ")
            };
        }
        println!("");
    }

    println!("");
    println!("");

    let mut sorted_clues = crossword.get_clues().collect::<Vec<_>>();
    sorted_clues.sort_by_key(|w| w.display_number);

    println!("Horizontal clues:");

    for clue in sorted_clues.iter().filter(|w| w.direction == ClueDirection::Horizontal) {
        println!("{}: {}", clue.display_number, clue.text);
    }

    println!("");

    println!("Vertical clues:");

    for clue in sorted_clues.iter().filter(|w| w.direction == ClueDirection::Vertical) {
        println!("{}: {}", clue.display_number, clue.text);
    }
}