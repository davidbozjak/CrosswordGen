use crate::crossword::Crossword;
use crate::crossword::Cell;
use crate::crossword::ClueDirection;
use crate::helpers;

pub fn get_crossword_fitness(crossword: &Crossword) -> f32 {
    let cells:Vec<&Cell> = crossword.get_cells().into_iter().collect();
    
    let max_possible = cells.iter().count() * 2;

    let actual = cells.iter().map(|w| w.get_clue_belonging_count()).sum::<usize>();

    return actual as f32 / max_possible as f32;
}

pub fn get_missing_words(crossword: &Crossword) -> Vec<String> {
    let mut missing_words: Vec<String> = Vec::new();
    
    for x in crossword.min_x()..crossword.max_x() {
        for y in crossword.min_y()..crossword.max_y() {
            
            let mut try_horizontal:bool = false;
            let mut try_vertical:bool = false;
            
            let cell = crossword.get_cellvalue_at(x, y);

            match cell {
                Some(crate::crossword::CellValue::Normal(_)) => {},
                Some(crate::crossword::CellValue::ClueStart(w)) => match w {
                    crate::crossword::ClueStartCell::Horizontal(_) => { try_vertical = true; },
                    crate::crossword::ClueStartCell::Vertical(_) => { try_horizontal = true; },
                    crate::crossword::ClueStartCell::HorizontalVertical(_) => {},
                },
                Some(crate::crossword::CellValue::ClueEnd()) => { try_horizontal = true; try_vertical = true; },
                None => { try_horizontal = true; try_vertical = true; }
            }

            if try_horizontal {
                let horizontal_str = get_word_from_cell(crossword, x, y, ClueDirection::Horizontal);

                if horizontal_str.len() > 0 {
                    missing_words.push(horizontal_str);
                }
            }

            if try_vertical {
                let vertical_str = get_word_from_cell(crossword, x, y, ClueDirection::Vertical);

                if vertical_str.len() > 0 {
                    missing_words.push(vertical_str);
                }
            }
        }
    }

    return missing_words;
}

fn get_word_from_cell(crossword: &Crossword, start_x: i32, start_y: i32, direction: crate::crossword::ClueDirection) -> String {
    let mut word = String::new();

    let (x_inc, y_inc) = helpers::get_incrementors_for_direction(direction);

    let mut x = x_inc(start_x);
    let mut y = y_inc(start_y);

    loop {
        let cell = crossword.get_cellvalue_at(x, y);

        if let Some(c) = cell {
            match c {
                crate::crossword::CellValue::Normal(w) => {
                    word.push_str(&w.to_string());
                }
                _ => { break; }
            }
        }
        else {
            break;
        }

        x = x_inc(x);
        y = y_inc(y);
    }

    return word;
}

