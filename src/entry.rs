use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    key: String,
    clue: String,
}

impl Entry {
        
    pub fn new(key: String, clue: String) -> Entry {
        Self {key: key, clue: clue}
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_key_at(&self, i: usize) -> Option<char> {
        self.key.chars().nth(i)
    }

    pub fn key_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.key.chars()
    }

    pub fn key_len(&self) -> usize {
        self.key.chars().count()
    }

    pub fn get_clue(&self) -> &str {
        &self.clue
    }

    pub fn clue_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.clue.chars()
    }

    pub fn clue_len(&self) -> usize {
        self.clue.chars().count()
    }
}
