use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    key: String,
    clue: String,
}

impl Entry {
    pub fn new(key: &str, clue: &str) -> Entry {
        let mut key_with_placeholder = format!("_{}_", key);

        Entry {
            key: key_with_placeholder,
            clue: clue.to_string(),
        }
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
        self.key.len()
    }

    pub fn get_clue(&self) -> &str {
        &self.clue
    }

    pub fn clue_chars(&self) -> impl Iterator<Item = char> + '_ {
        self.clue.chars()
    }

    pub fn clue_len(&self) -> usize {
        self.clue.len()
    }
}
