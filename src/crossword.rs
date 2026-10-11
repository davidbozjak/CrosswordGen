use crate::entry::Entry;
use crate::random_provider::RandomProvider;

pub enum CellValue {
    Normal(char),
    ClueId(u32),
    ClueEnd()
}

struct Cell {
    pos_x: i32,
    pos_y: i32,
    value: CellValue
}

pub struct Clue {
    pub id: u32,
    pub text: String
}

pub struct Crossword {
    elements: Vec<Cell>,
    clues: Vec<Clue>
}

impl Crossword
{
    pub fn generate<I>(entries: I, random_provider: &mut impl RandomProvider) -> Crossword 
    where 
        I: IntoIterator<Item = Entry>,
        Entry: Clone
    {
        let mut entries_working_copy: Vec<Entry> = entries.into_iter().collect();
        
        let first_i = random_provider.next_u32() as usize % entries_working_copy.len();
        let first = entries_working_copy[first_i].clone();
        entries_working_copy.remove(first_i);

        Self::generate_with_first(&first, entries_working_copy, random_provider)
    }

    pub fn generate_with_first<I>(first: &Entry, entries: I, random_provider: &mut impl RandomProvider) -> Crossword 
    where 
        I: IntoIterator<Item = Entry>,
        Entry: Clone
    {
        let mut crossword = Crossword {elements: Vec::new(), clues: Vec::new()};

        crossword.insert_entry(&first, 0, 0, |w| w + 1, |w| w);

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

        for y in min_y .. max_y {
            for x in min_x .. max_x {
                if self.try_fit(&entry, x, y, |w| w + 1, |w| w)
                {
                    self.insert_entry(&entry, x, y, |w| w + 1, |w| w);

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

        // first needs to be empty
        let first = self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y);
        x = x_incrementor(x);
        y = y_incrementor(y);

        if first.is_some()
        {
            // Future improvement - handle 2 cases to make it more complex
            // Clue_Id -> make it possible for 2 clues to start on the same square
            // Clue_End -> replace clue end with new clue start
            return false;
        }

        while counter < entry.key_len() {
            let element_at_pos = self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y);

            if element_at_pos.is_some() {
                match element_at_pos.unwrap().value {
                    CellValue::ClueId(_) => 
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
        let last = self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y);
        match last {
            Some(w) => match w.value {
                CellValue::ClueId(_) => {}
                CellValue::ClueEnd() => {}
                CellValue::Normal(_) => { return false; }
            }
            None => {}
        }

        //fit only successful if we actually found an intersection
        has_intersection
    }

    fn insert_entry<T1, T2>(&mut self, entry: &Entry, start_x: i32, start_y: i32, x_incrementor: T1, y_incrementor: T2)
    where
        T1: Fn(i32)->i32,
        T2: Fn(i32)->i32
    {
        let clue_id = self.clues.iter().count() as u32 + 1;
        self.clues.push(Clue {id: clue_id, text: entry.get_clue().to_string()});

        let mut counter:usize = 0;
        let mut x = start_x;
        let mut y = start_y;

        self.elements.push(Cell {pos_x: x, pos_y: y, value: CellValue::ClueId(clue_id)});
        x = x_incrementor(x);
        y = y_incrementor(y);

        while counter < entry.key_len() {
            let element_at_pos = self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y);

            if element_at_pos.is_none() {
                self.elements.push(Cell {pos_x: x, pos_y: y, value: CellValue::Normal(entry.get_key_at(counter).unwrap())});
            }
            else {
                match element_at_pos.unwrap().value {
                    CellValue::Normal(w) => assert!(Some(w) == entry.get_key_at(counter)),
                    CellValue::ClueEnd() => assert!(true),
                    CellValue::ClueId(_) => assert!(false)
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

    pub fn get_char_at(&self, x: i32, y:i32) -> Option<&CellValue> {
        match self.elements.iter().find(|w|w.pos_x == x && w.pos_y == y) {
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
}