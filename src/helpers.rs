use crate::crossword::Crossword;
use crate::crossword::ClueDirection;

pub fn get_incrementors_for_direction(direction: ClueDirection) -> (fn(i32)->i32, fn(i32)->i32) {
    match direction {
        ClueDirection::Horizontal => (|w| w + 1, |w| w),
        ClueDirection::Vertical => (|w| w, |w| w + 1)
    }
}

pub fn get_missing_words(crossword: &Crossword) -> Vec<String> {
    let mut missing_words: Vec<String> = Vec::new();
    
    // Start above and to the left, but no need to extend outside of bottom right, as we only increment when searching for words
    let min_x = crossword.min_x() - 1;
    let max_x = crossword.max_x() + 1;
    let min_y = crossword.min_y() - 1;
    let max_y = crossword.max_y() + 1;

    for x in min_x..max_x {
        for y in min_y..max_y {
            
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

pub fn get_all_words(crossword: &Crossword) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    
    let min_x = crossword.min_x() - 1;
    let max_x = crossword.max_x() + 1;
    let min_y = crossword.min_y() - 1;
    let max_y = crossword.max_y() + 1;

    for x in min_x..max_x {
        for y in min_y..max_y {
            
            let cell = crossword.get_cellvalue_at(x, y);

            match cell {
                Some(crate::crossword::CellValue::Normal(_)) => {},
                _ => { 
                    
                    let horizontal_str = get_word_from_cell(crossword, x, y, ClueDirection::Horizontal);

                    if horizontal_str.len() > 0 {
                        words.push(horizontal_str);
                    }
                
                    let vertical_str = get_word_from_cell(crossword, x, y, ClueDirection::Vertical);

                    if vertical_str.len() > 0 {
                        words.push(vertical_str);
                    }
                }
            }
        }
    }

    return words;
}

fn get_word_from_cell(crossword: &Crossword, start_x: i32, start_y: i32, direction: crate::crossword::ClueDirection) -> String {
    let mut word = String::new();

    let (x_inc, y_inc) = get_incrementors_for_direction(direction);

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

#[cfg(test)]
mod tests {
    use crate::random_provider::FakeRandomProvider;

    use super::*;

    #[test]
    fn get_all_words_provides_words_even_when_crossword_is_single_entry()
    {
        let entries = [
            crate::Entry::new("abc".to_string(), "abc".to_string(), None),
        ];

        let crossword = Crossword::generate(100, 100, entries, & mut FakeRandomProvider::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]));

        //crate::crossword_printer::print(&crossword);

        let words = get_all_words(&crossword);

        assert!(words.len() == 4);
    }

    #[test]
    fn get_missing_words_provides_words_even_when_crossword_is_single_entry()
    {
        let entries = [
            crate::Entry::new("abc".to_string(), "abc".to_string(), None),
        ];

        let crossword = Crossword::generate(100, 100, entries, & mut FakeRandomProvider::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]));

        //crate::crossword_printer::print(&crossword);

        let words = get_missing_words(&crossword);

        assert!(words.len() == 3);
    }

    #[test]
    fn all_missing_words_are_returned_by_get_all_words()
    {
        let entries = [
            crate::Entry::new("abcd".to_string(), "abcd".to_string(), None),
            crate::Entry::new("bef".to_string(), "bef".to_string(), None),
            crate::Entry::new("gdhi".to_string(), "gdhi".to_string(), None),
            crate::Entry::new("jgkl".to_string(), "jgkl".to_string(), None),
        ];

        let crossword = Crossword::generate(100, 100, entries, & mut FakeRandomProvider::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]));

        //crate::crossword_printer::print(&crossword);

        let missing_words = get_missing_words(&crossword);
        let all_words = get_all_words(&crossword);

        assert!(missing_words.iter().all(|w|all_words.contains(w)));
    }

    #[test]
    fn all_entries_are_returned_by_get_all_words()
    {
        // tests that a clue that doesn't fit first time it is picked ("e") gets reused later.
        // seems obvious now but first version of the crossword creation algorithm didn't work that way and this test failed - only clue 1 and 3 were added.

        let entries = [
            crate::Entry::new("abcd".to_string(), "abcd".to_string(), None),
            crate::Entry::new("bef".to_string(), "bef".to_string(), None),
            crate::Entry::new("gdhi".to_string(), "gdhi".to_string(), None),
            crate::Entry::new("jgkl".to_string(), "jgkl".to_string(), None),
        ];

        let crossword = Crossword::generate(100, 100, entries.clone(), & mut FakeRandomProvider::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]));

        //crate::crossword_printer::print(&crossword);

        let all_words = get_all_words(&crossword);

        assert!(entries.iter().all(|w|all_words.contains(&w.get_key().to_string())));
    }

    #[test]
    fn none_entries_are_returned_by_get_missing_words()
    {
        // tests that a clue that doesn't fit first time it is picked ("e") gets reused later.
        // seems obvious now but first version of the crossword creation algorithm didn't work that way and this test failed - only clue 1 and 3 were added.

        let entries = [
            crate::Entry::new("abcd".to_string(), "abcd".to_string(), None),
            crate::Entry::new("bef".to_string(), "bef".to_string(), None),
            crate::Entry::new("gdhi".to_string(), "gdhi".to_string(), None),
            crate::Entry::new("jgkl".to_string(), "jgkl".to_string(), None),
        ];

        let crossword = Crossword::generate(100, 100, entries.clone(), & mut FakeRandomProvider::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]));

        //crate::crossword_printer::print(&crossword);

        let missing_words = get_missing_words(&crossword);

        assert!(!entries.iter().any(|w|missing_words.contains(&w.get_key().to_string())));
    }
}