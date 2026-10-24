use crate::entry::Entry;
use crate::random_provider::RandomProvider;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClueStartCell
{
    Horizontal(u32),
    Vertical(u32),
    HorizontalVertical(u32)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellValue {
    Normal(char),
    ClueStart(ClueStartCell),
    ClueEnd()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClueDirection {
    Horizontal,
    Vertical
}

struct Cell {
    pos_x: i32,
    pos_y: i32,
    value: CellValue
}

pub struct Clue {
    pub id: u32,
    pub text: String,
    pub direction: ClueDirection
}

pub struct Crossword {
    elements: Vec<Cell>,
    clues: Vec<Clue>,
    limit_x: u8,
    limit_y: u8
}

impl Crossword
{
    pub fn generate<I>(limit_x: u8, limit_y: u8, entries: I, random_provider: &mut impl RandomProvider) -> Crossword 
    where 
        I: IntoIterator<Item = Entry>,
        Entry: Clone
    {
        let mut entries_working_copy: Vec<Entry> = entries.into_iter().collect();
        
        let first_i = random_provider.next_u32() as usize % entries_working_copy.len();
        let first = entries_working_copy[first_i].clone();
        entries_working_copy.remove(first_i);

        Self::generate_with_first(limit_x, limit_y, &first, entries_working_copy, random_provider)
    }

    pub fn generate_with_first<I>(limit_x: u8, limit_y: u8, first: &Entry, entries: I, random_provider: &mut impl RandomProvider) -> Crossword 
    where 
        I: IntoIterator<Item = Entry>,
        Entry: Clone
    {
        let mut crossword = Crossword {elements: Vec::new(), clues: Vec::new(), limit_x: limit_x, limit_y: limit_y};

        crossword.insert_entry(&first, 0, 0, ClueDirection::Horizontal);

        let mut entries_working_copy: Vec<Entry> = entries.into_iter().collect();
        // algoritm pseudocode:
        while entries_working_copy.len() > 0 {

            let next_i = random_provider.next_u32() as usize % entries_working_copy.len();
            let next = entries_working_copy[next_i].clone();
            entries_working_copy.remove(next_i);
            
            crossword.try_add_entry(&next);

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

        for y in min_y .. max_y {
            for x in min_x .. max_x {
                if self.try_fit(entry, x, y, ClueDirection::Vertical)
                {
                    self.insert_entry(entry, x, y, ClueDirection::Vertical);

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

        for y in min_y .. max_y {
            for x in min_x .. max_x {
                if self.try_fit(&entry, x, y, ClueDirection::Horizontal)
                {
                    self.insert_entry(&entry, x, y, ClueDirection::Horizontal);

                    return true;
                }
            }
        }

        false
    }

    fn try_fit(&self, entry: &Entry, start_x: i32, start_y: i32, direction: ClueDirection) -> bool
    {
        let (x_incrementor, y_incrementor) = Crossword::get_incrementors_for_direction(direction);

        let mut counter:usize = 0;
        let mut x = start_x;
        let mut y = start_y;

        let mut has_intersection = false;

        // first needs to be empty
        let first = self.get_cell_at(x, y);
        x = x_incrementor(x);
        y = y_incrementor(y);

        if let Some(first) = first {
            // Future improvement
            // Clue_End -> replace clue end with new clue start
            match first.value {
                CellValue::ClueEnd() => return false,
                CellValue::Normal(_) => return false,
                CellValue::ClueStart(start_value) => {
                    // return false if it is the same direction or if it's already both. If it's oposite then continue
                    match direction
                    {
                        ClueDirection::Horizontal => {
                            match start_value {
                                ClueStartCell::Horizontal(_) => return false,
                                ClueStartCell::Vertical(_) => {},
                                ClueStartCell::HorizontalVertical(_) => return false,
                            }
                        },
                        ClueDirection::Vertical => {
                            match start_value {
                                ClueStartCell::Horizontal(_) => {},
                                ClueStartCell::Vertical(_) => return false,
                                ClueStartCell::HorizontalVertical(_) => return false,
                            }
                        }
                    }
                }
            }
        }

        while counter < entry.key_len() {
            let element_at_pos = self.get_cell_at(x, y);

            if element_at_pos.is_some() {
                match element_at_pos.unwrap().value {
                    CellValue::ClueStart(_) => 
                    {
                        return false;
                    },
                    CellValue::ClueEnd() => {
                        return false;
                    }
                    CellValue::Normal(character) =>
                    {
                        if Some(character) == entry.get_key_at(counter) {
                            has_intersection = true;
                        }
                        else {
                            return false;
                        }
                    }
                }
            }

            x = x_incrementor(x);
            y = y_incrementor(y);
            counter += 1;
        }

        // last needs to be not normal (both start and end is OK)
        let last = self.get_cell_at(x, y);
        if let Some(w) = last {
            if let CellValue::Normal(_) = w.value {
                return false;
            }
        }

        // check that the crossword hasn't grown out of limits
        let new_min_y = [self.min_y(), start_y, y].into_iter().min().unwrap();
        let new_min_x = [self.min_x(), start_x, x].into_iter().min().unwrap();
        let new_max_y = [self.max_y(), start_y, y].into_iter().max().unwrap();
        let new_max_x = [self.max_x(), start_x, x].into_iter().max().unwrap();
        
        let new_potential_max_height = new_max_y - new_min_y + 1;
        let new_potential_max_width = new_max_x - new_min_x + 1;
        
        if new_potential_max_height > (self.limit_y as i32) ||
            new_potential_max_width > (self.limit_x as i32) {
            return false;
        }

        //fit only successful if we actually found an intersection
        has_intersection
    }

    fn insert_entry(&mut self, entry: &Entry, start_x: i32, start_y: i32, direction: ClueDirection)
    {
        let (x_incrementor, y_incrementor) = Crossword::get_incrementors_for_direction(direction);

        // assign the preliminary id, but it might change. We will want to insert either way but it could be that we are intersecting with an existing clue and we will want to use that id instead of creating a new one
        let mut clue_id = self.clues.iter().count() as u32 + 1;
        
        let mut counter:usize = 0;
        let mut x = start_x;
        let mut y = start_y;

        let first = self.get_mut_cell_at(x, y);

        if let Some(first) = first {
            match first.value {
                CellValue::ClueEnd() => panic!("Not supporting replacing end with start (yet)"),
                CellValue::Normal(_) => panic!("Should never happen, bug in try_fit logic"),
                CellValue::ClueStart(start_value) => {
                    match direction
                    {
                        ClueDirection::Horizontal => {
                            match start_value {
                                ClueStartCell::Horizontal(_) => panic!("Horizontal already filled, should never happen, bug in try_fit logic"),
                                ClueStartCell::Vertical(vertical_id) => {
                                    clue_id = vertical_id;
                                    first.value = CellValue::ClueStart(ClueStartCell::HorizontalVertical(clue_id));
                                },
                                ClueStartCell::HorizontalVertical(_) => panic!("Both already filled, should never happen, bug in try_fit logic"),
                            }
                        },
                        ClueDirection::Vertical => {
                            match start_value {
                                ClueStartCell::Horizontal(horizontal_id) => {
                                    clue_id = horizontal_id;
                                    first.value = CellValue::ClueStart(ClueStartCell::HorizontalVertical(clue_id));
                                },
                                ClueStartCell::Vertical(_) => panic!("Vertical already filled, should never happen, bug in try_fit logic"),
                                ClueStartCell::HorizontalVertical(_) => panic!("Both already filled, should never happen, bug in try_fit logic"),
                            }
                        }
                    }
                }
            }
        }
        else {
            match direction {
                ClueDirection::Horizontal => self.elements.push(Cell {pos_x: x, pos_y: y, value: CellValue::ClueStart(ClueStartCell::Horizontal(clue_id))}),
                ClueDirection::Vertical => self.elements.push(Cell {pos_x: x, pos_y: y, value: CellValue::ClueStart(ClueStartCell::Vertical(clue_id))})
            };
        }

        self.clues.push(Clue {id: clue_id, text: entry.get_clue().to_string(), direction});

        // First element was handled above, iterate once before handling next elements
        x = x_incrementor(x);
        y = y_incrementor(y);

        while counter < entry.key_len() {
            let element_at_pos = self.get_cell_at(x, y);

            if element_at_pos.is_none() {
                self.elements.push(Cell {pos_x: x, pos_y: y, value: CellValue::Normal(entry.get_key_at(counter).unwrap())});
            }
            else {
                match element_at_pos.unwrap().value {
                    CellValue::Normal(w) => assert!(Some(w) == entry.get_key_at(counter)),
                    CellValue::ClueEnd() => {},
                    CellValue::ClueStart(_) => panic!("Should never happen, bug in try_fit logic"),
                };
            }

            x = x_incrementor(x);
            y = y_incrementor(y);
            counter += 1;
        }

        self.elements.push(Cell {pos_x: x, pos_y: y, value: CellValue::ClueEnd()});
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

    fn get_mut_cell_at(&mut self, x: i32, y:i32) -> Option<&mut Cell> {
        match self.elements.iter_mut().find(|w|w.pos_x == x && w.pos_y == y) {
            Some(w) => Some(w),
            None => None
        }
    }

    fn get_mut_cellvalue_at(&mut self, x: i32, y:i32) -> Option<&mut CellValue> {
        match self.get_mut_cell_at(x, y) {
            Some(w) => Some(&mut w.value),
            None => None
        }
    }

    fn get_cell_at(&self, x: i32, y:i32) -> Option<&Cell> {
        match self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y) {
            Some(w) => Some(w),
            None => None
        }
    }

    pub fn get_cellvalue_at(&self, x: i32, y:i32) -> Option<&CellValue> {
        match self.get_cell_at(x, y) {
            Some(w) => Some(&w.value),
            None => None
        }
    }

    pub fn get_clues(&self) -> impl Iterator<Item = &Clue> + '_ {
        self.clues.iter()
    }

    pub fn get_no_rows(&self) -> u8 {
        let no_rows = self.max_y() - self.min_y() + 1;
        assert!(no_rows > 0);
        no_rows as u8
    }

    pub fn get_no_cols(&self) -> u8 {
        let no_cols = self.max_x() - self.min_x() + 1;
        assert!(no_cols > 0);
        no_cols as u8
    }

    pub fn get_incrementors_for_direction(direction: ClueDirection) -> (fn(i32)->i32, fn(i32)->i32) {
        match direction {
            ClueDirection::Horizontal => (|w| w + 1, |w| w),
            ClueDirection::Vertical => (|w| w, |w| w + 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::random_provider::FakeRandomProvider;

use super::*;

    #[test]
    fn adds_horizontalvertical_clue_horizontalfirst()
    {
        let entries = [
            Entry::new("abc".to_string(), "abc".to_string()),
            Entry::new("cdef".to_string(), "cdef".to_string()),
            Entry::new("dghi".to_string(), "dghi".to_string()),
            Entry::new("jiklm".to_string(), "jiklm".to_string()),
            Entry::new("jnop".to_string(), "jnop".to_string()),
            Entry::new("hrs".to_string(), "hrs".to_string()),
        ];

        let crossword = Crossword::generate(100, 100, entries, & mut FakeRandomProvider::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]));

        //crate::crossword_printer::print(&crossword);

        let clues = crossword.get_clues().collect::<Vec<&Clue>>();
        assert!(clues.iter().count() == 6);
        assert!(clues.iter().all(|w| w.id >= 1 && w.id <= 5));   //the ID 5 is double

        let cellvalue = crossword.get_cellvalue_at(5, 0);
        assert!((cellvalue.is_some()));
        assert!((cellvalue.unwrap() == &CellValue::ClueStart(ClueStartCell::HorizontalVertical(5))));
    }

    #[test]
    fn adds_horizontalvertical_clue_verticalfirst()
    {
        let entries = [
            Entry::new("abcd".to_string(), "abcd".to_string()),
            Entry::new("bef".to_string(), "bef".to_string()),
            Entry::new("gdhi".to_string(), "gdhi".to_string()),
            Entry::new("jgkl".to_string(), "jgkl".to_string()),
        ];

        let crossword = Crossword::generate(100, 100, entries, & mut FakeRandomProvider::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]));

        //crate::crossword_printer::print(&crossword);

        let clues = crossword.get_clues().collect::<Vec<&Clue>>();
        assert!(clues.iter().count() == 4);
        assert!(clues.iter().all(|w| w.id >= 1 && w.id <= 3));   //the ID 5 is double

        let cellvalue = crossword.get_cellvalue_at(2, -1);
        assert!((cellvalue.is_some()));
        assert!((cellvalue.unwrap() == &CellValue::ClueStart(ClueStartCell::HorizontalVertical(2))));
    }
}