use std::collections::HashMap;

pub type WordsStats = HashMap<String, i16>;
pub type LettersStats = HashMap<char, i16>;
pub struct Statistics {
    words: WordsStats,
    letters: LettersStats,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            letters: HashMap::new(),
        }
    }
}
