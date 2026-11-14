use crate::crossword::Crossword;
use crate::crossword::Cell;
use crate::helpers;

pub fn get_crossword_fitness(crossword: &Crossword) -> f32 {
    
    // each score function is expected to return a value between 0.0 and 1.0, where 1.0 is the best possible score and 0.0 is the worst possible score.

    let cross_connected_score = get_cross_connected_score(crossword);
    let clue_len_score = get_clue_len_score(crossword);

    let scores = [cross_connected_score, clue_len_score];

    assert!(scores.iter().all(|w|(0.0..=1.0).contains(w)));   

    let average_score = scores.iter().sum::<f32>() / scores.len() as f32;
    
    return average_score;
}

fn get_cross_connected_score(crossword: &Crossword) -> f32 {
    let cells:Vec<&Cell> = crossword.get_cells().into_iter().collect();
    
    let max_possible = cells.iter().count() * 2;

    let actual = cells.iter().map(|w| w.get_clue_belonging_count()).sum::<usize>();

    return actual as f32 / max_possible as f32;
}

fn get_clue_len_score(crossword: &Crossword) -> f32 {
    let words = helpers::get_all_words(crossword);
    let lengths = words.into_iter().map(|w| w.chars().count()).collect::<Vec<usize>>();
    
    let count_over_1 = lengths.iter().filter(|w| **w > 1).count();
    let score = count_over_1 as f32 / lengths.len() as f32;

    return score;
}
