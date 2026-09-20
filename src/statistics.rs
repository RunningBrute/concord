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

pub fn get_all_stats(words: &[&str]) -> Statistics {
    let mut result: Statistics = Statistics::new();

    get_words_stats(words, &mut result.words_mut());
    get_letters_stats(words, &mut result.letters_mut());

    return result;
}

fn get_words_stats(words: &[&str], mut stats: &mut BTreeMap<String, i16>) {
    // words.swap_remove(0);

    for &word in words {
        match stats.get_mut(word) {
            Some(value) => update_word_frequency(&word, value),
            None => add_new_word_to_stats(word, &mut stats),
        }
    }
}

fn get_letters_stats(words: &[&str], stats: &mut BTreeMap<char, i16>) {
    for word in words {
        for letter in word.chars() {
            match stats.get_mut(&letter) {
                Some(value) => {
                    *value = *value + 1;
                    println!("Letter already exist: {}, {}", letter, *value);
                }
                None => {
                    println!("New letter added: {}", letter);
                    stats.insert(letter, 1);
                }
            }
        }
    }
}

fn update_word_frequency(word: &str, frequency: &mut i16) {
    *frequency = *frequency + 1;
    println!("Word already exist: {}, {}", word, *frequency);
}

fn add_new_word_to_stats(word: &str, stats: &mut BTreeMap<String, i16>) {
    println!("New word added: {}", word);
    stats.insert(word.to_string(), 1);
}
