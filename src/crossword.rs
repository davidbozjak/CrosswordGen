use crate::entry::Entry;
use crate::random_provider::RandomProvider;

struct Cell {
    pos_x: i32,
    pos_y: i32,
    char: char
}

pub struct Crossword {
    elements: Vec<Cell>,
}

impl Crossword
{
    pub fn generate(entries: &[&Entry], random_provider: &mut impl RandomProvider) -> Crossword {
        let mut entries_working_copy = entries.to_vec();
        
        let first_i = random_provider.next_u32() as usize % entries_working_copy.len();
        let first = entries_working_copy[first_i];
        entries_working_copy.remove(first_i);

        Self::generate_with_first(first, &entries_working_copy, random_provider)
    }

    pub fn generate_with_first(first: &Entry, entries: &[&Entry], random_provider: &mut impl RandomProvider) -> Crossword {
        let mut cells:Vec<Cell> = Vec::new();
        
        let mut entries_working_copy = entries.to_vec();

        let mut i: i32 = 0;
        for c in first.key_chars()
        {
            cells.push(Cell { pos_x: i, pos_y: 0, char:c});
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
        let min_x = self.min_x();
        let max_x = self.max_x() + 1;
        let min_y = self.min_y() - entry.key_len() as i32;
        let max_y = self.max_y() + 1;

        for x in min_x .. max_x {
            for y in min_y .. max_y {
                if self.try_fit(entry, x, y, |w| w, |w| w + 1)
                {
                    self.insert_entry(entry, x, y, |w| w, |w| w + 1);

                    return true;
                }
            }
        }

        false
    }

    fn try_add_horizontal(&mut self, entry: &Entry) -> bool {
        let min_x = self.min_x() - entry.key_len() as i32;
        let max_x = self.max_x() + 1;
        let min_y = self.min_y();
        let max_y = self.max_y() + 1;

        for x in min_x .. max_x {
            for y in min_y .. max_y {
                if self.try_fit(entry, x, y, |w| w, |w| w + 1)
                {
                    self.insert_entry(entry, x, y, |w| w, |w| w + 1);

                    return true;
                }
            }
        }

        false
    }

    fn try_fit<T1, T2>(&self, entry: &Entry, start_x: i32, start_y: i32, x_incrementor: T1, y_incrementor: T2) -> bool
    where
        T1: Fn(i32)->i32,
        T2: Fn(i32)->i32
    {
        let mut counter:usize = 0;
        let mut x = start_x;
        let mut y = start_y;

        let mut has_intersection = false;

        while counter < entry.key_len() {
            let element_at_pos = self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y);

            if element_at_pos.is_some() {
                if Some(element_at_pos.unwrap().char) == entry.get_at(counter)
                {
                    //do not allow matching on the first element of the clue ("_")
                    if counter > 0 
                    {
                        has_intersection = true;
                    }
                }
                else {
                    return false;
                }
            }

            x = x_incrementor(x);
            y = y_incrementor(y);
            counter += 1;
        }

        has_intersection
    }

    fn insert_entry<T1, T2>(&mut self, entry: &Entry, start_x: i32, start_y: i32, x_incrementor: T1, y_incrementor: T2)
    where
        T1: Fn(i32)->i32,
        T2: Fn(i32)->i32
    {
        let mut counter:usize = 0;
        let mut x = start_x;
        let mut y = start_y;

        while counter < entry.key_len() {
            let element_at_pos = self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y);

            if element_at_pos.is_none() {
                self.elements.push(Cell {pos_x: x, pos_y: y, char: entry.get_at(counter).unwrap()});
            }
            else {
                assert!(Some(element_at_pos.unwrap().char) == entry.get_at(counter))
            }

            x = x_incrementor(x);
            y = y_incrementor(y);
            counter += 1;
        }
    }

    pub fn max_x(&self) -> i32 {
        match self.elements.iter().max_by_key(|w| w.pos_x) {
            Some(u) => u.pos_x,
            None => 0
        }
    }

    pub fn min_x(&self) -> i32 {
        match self.elements.iter().min_by_key(|w| w.pos_x) {
            Some(u) => u.pos_x,
            None => 0
        }
    }

    pub fn max_y(&self) -> i32 {
        match self.elements.iter().max_by_key(|w| w.pos_y) {
            Some(u) => u.pos_y,
            None => 0
        }
    }

    pub fn min_y(&self) -> i32 {
        match self.elements.iter().min_by_key(|w| w.pos_y) {
            Some(u) => u.pos_y,
            None => 0
        }
    }

    pub fn get_char_at(&self, x: i32, y:i32) -> Option<char> {
        match self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y) {
            Some(w) => Some(w.char),
            None => None
        }
    }
}