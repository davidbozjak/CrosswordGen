use crate::crossword::Crossword;
use crate::crossword::Cell;

pub fn get_crossword_fitness(crossword: &Crossword) -> f32 {
    let cells:Vec<&Cell> = crossword.get_cells().into_iter().collect();
    
    let max_possible = cells.iter().count() * 2;

    let actual = cells.iter().map(|w| w.get_clue_belonging_count()).sum::<usize>();

    return actual as f32 / max_possible as f32;
}
