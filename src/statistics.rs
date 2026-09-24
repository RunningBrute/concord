pub mod word;
pub mod letter;

pub struct Statistics {
    words: word::WordsStats,
    letters: letter::LettersStats,
}

impl Statistics {
    pub fn from(words: &[&str]) -> Self {
        Self {
            words: word::get_words_stats(words),
            letters: letter::get_letters_stats(words),
        }
    }

    pub fn words(&self) -> &word::WordsStats {
        return &self.words;
    }

    pub fn letters(&self) -> &letter::LettersStats {
        return &self.letters;
    }

    pub fn _words_mut(&mut self) -> &mut word::WordsStats {
        return &mut self.words;
    }

    pub fn _letters_mut(&mut self) -> &mut letter::LettersStats {
        return &mut self.letters;
    }
}