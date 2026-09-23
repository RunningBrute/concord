use std::collections::HashMap;

pub type WordsStats = HashMap<String, i16>;
pub type LettersStats = HashMap<char, i16>;
pub struct Statistics {
    words: WordsStats,
    letters: LettersStats,
}

impl Statistics {
    pub fn from(words: &[&str]) -> Self {
        Self {
            words: get_words_stats(words),
            letters: get_letters_stats(words),
        }
    }

    pub fn words(&self) -> &WordsStats {
        return &self.words;
    }

    pub fn letters(&self) -> &LettersStats {
        return &self.letters;
    }

    pub fn _words_mut(&mut self) -> &mut WordsStats {
        return &mut self.words;
    }

    pub fn _letters_mut(&mut self) -> &mut LettersStats {
        return &mut self.letters;
    }
}

fn get_words_stats(words: &[&str]) -> HashMap<String, i16> {
    // words.swap_remove(0);
    let mut stats = HashMap::new();

    for &word in words {
        match stats.get_mut(word) {
            Some(value) => update_word_frequency(&word, value),
            None => add_new_word_to_stats(word, &mut stats),
        }
    }

    stats
}

fn get_letters_stats(words: &[&str]) -> HashMap<char, i16> {
    let mut stats = HashMap::new();

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

    stats
}

/// Function update `frequency` parameter by one each time
/// when function `update_word_frequency` is called.
/// 
/// ```
/// use concord::statistics::update_word_frequency;
/// 
/// let mut frequency: i16 = 0;
/// let word: String = String::from("hello");
/// update_word_frequency(&word, &mut frequency);
/// assert_eq!(frequency, 1);
/// ```
fn update_word_frequency(word: &str, frequency: &mut i16) {
    *frequency = *frequency + 1;
    println!("Word already exist: {}, {}", word, *frequency);
}

fn add_new_word_to_stats(word: &str, stats: &mut HashMap<String, i16>) {
    println!("New word added: {}", word);
    stats.insert(word.to_string(), 1);
}
