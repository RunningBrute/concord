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

    pub fn words(&self) -> &WordsStats {
        return &self.words;
    }

    pub fn letters(&self) -> &LettersStats {
        return &self.letters;
    }

    pub fn words_mut(&mut self) -> &mut WordsStats {
        return &mut self.words;
    }

    pub fn letters_mut(&mut self) -> &mut LettersStats {
        return &mut self.letters;
    }
}
