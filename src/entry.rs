use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    key: String,
    clue: String,
    ai_gen_confidence: Option<u32>
}

impl Entry {
        
    pub fn new(key: String, clue: String, ai_gen_confidence: Option<u32>) -> Entry {
        Self {key: key, clue: clue, ai_gen_confidence:ai_gen_confidence}
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_key_at(&self, i: usize) -> Option<char> {
        self.key.chars().nth(i)
    }

    pub fn key_chars(&self) -> impl Iterator<Item = char>{
        self.key.chars()
    }

    pub fn key_len(&self) -> usize {
        self.key.chars().count()
    }

    pub fn get_clue(&self) -> &str {
        &self.clue
    }

    pub fn clue_chars(&self) -> impl Iterator<Item = char>{
        self.clue.chars()
    }

    pub fn clue_len(&self) -> usize {
        self.clue.chars().count()
    }

    pub fn get_confidence(&self) -> u32 {
        match self.ai_gen_confidence {
            None => 0,
            Some(w) => w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_len_matches()
    {
        let t = Entry::new("abcd".to_string(), "_".to_string(), None);
        assert_eq!(t.key_len(), 4);
    }

    #[test]
    fn key_len_matches_with_se_chars()
    {
        let t = Entry::new("öåä1".to_string(), "_".to_string(), None);
        assert_eq!(t.key_len(), 4);
    }

    #[test]
    fn key_len_matches_with_si_chars()
    {
        let t = Entry::new("ščž1".to_string(), "_".to_string(), None);
        assert_eq!(t.key_len(), 4);
    }

    #[test]
    fn clue_len_matches()
    {
        let t = Entry::new("_".to_string(), "abcd".to_string(), None);
        assert_eq!(t.clue_len(), 4);
    }

    #[test]
    fn clue_len_matches_with_se_chars()
    {
        let t = Entry::new("_".to_string(), "öåä1".to_string(), None);
        assert_eq!(t.clue_len(), 4);
    }

    #[test]
    fn clue_len_matches_with_si_chars()
    {
        let t = Entry::new("_".to_string(), "ščž1".to_string(), None);
        assert_eq!(t.clue_len(), 4);
    }
}