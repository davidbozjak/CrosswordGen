use crate::entry::Entry;
use crate::random_provider::RandomProvider;

struct Cell {
    posX: i32,
    posY: i32,
    char: char
}

pub struct Crossword {
    elements: Vec<Cell>,
}

impl Crossword
{
    pub fn Generate(entries: &[&Entry], random_provider: &mut impl RandomProvider) -> Crossword {
        let mut entries_working_copy = entries.to_vec();
        
        let first_i = random_provider.next_u32() as usize % entries_working_copy.len();
        let first = entries_working_copy[first_i];
        entries_working_copy.remove(first_i);

        Self::GenerateWithFirst(first, &entries_working_copy, random_provider)
    }

    pub fn GenerateWithFirst(first: &Entry, entries: &[&Entry], random_provider: &mut impl RandomProvider) -> Crossword {
        let mut cells:Vec<Cell> = Vec::new();
        
        let mut entries_working_copy = entries.to_vec();

        let mut i: i32 = 0;
        for c in first.key_chars()
        {
            cells.push(Cell { posX: i, posY: 0, char:c});
            i += 1;
        }

        let mut crossword = Crossword {elements: cells};

        // algoritm pseudocode:
        while entries_working_copy.len() > 0 {

            let next_i = random_provider.next_u32() as usize % entries_working_copy.len();
            let next = entries_working_copy[next_i];
            entries_working_copy.remove(next_i);
            
            crossword.try_add_entry(next);

        }

        return crossword;
    }

    fn try_add_entry(&mut self, entry: &Entry) -> bool {
        if self.try_add_vertical(entry) { true } else { self.try_add_horizontal(entry) }
    }

    fn try_add_vertical(&mut self, entry: &Entry) -> bool {
        false
    }

    fn try_add_horizontal(&mut self, entry: &Entry) -> bool {
        false
    }

    fn max_x(&self) -> i32 {
        match self.elements.iter().max_by_key(|w| w.posX) {
            Some(u) => u.posX,
            None => 0
        }
    }

    fn min_x(&self) -> i32 {
        match self.elements.iter().min_by_key(|w| w.posX) {
            Some(u) => u.posX,
            None => 0
        }
    }

    fn max_y(&self) -> i32 {
        match self.elements.iter().max_by_key(|w| w.posY) {
            Some(u) => u.posY,
            None => 0
        }
    }

    fn min_y(&self) -> i32 {
        match self.elements.iter().min_by_key(|w| w.posY) {
            Some(u) => u.posY,
            None => 0
        }
    }
}