#[derive(Debug)]
pub struct Entry {
    key: String,
    clue: String,
}

impl Entry {
    pub fn new(key: &str, clue: &str) -> Entry {
        Entry {
            key: key.to_string(),
            clue: clue.to_string(),
        }
    }

    pub fn get_at(&self, i: usize) -> Option<char> {
        self.key.chars().nth(i)
    }

    pub fn key_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.key.chars()
    }

    pub fn clue_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.clue.chars()
    }

    pub fn key_len(&self) -> usize {
        self.key.len()
    }

    pub fn clue_len(&self) -> usize {
        self.clue.len()
    }
}
