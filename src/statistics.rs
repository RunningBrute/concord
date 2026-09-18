use std::collections::BTreeMap;

pub type WordsStats = BTreeMap<String, i16>;
pub type LettersStats = BTreeMap<char, i16>;
pub struct Statistics {
    words: WordsStats,
    letters: LettersStats,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            words: BTreeMap::new(),
            letters: BTreeMap::new(),
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
