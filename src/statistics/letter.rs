use std::collections::HashMap;

pub type LettersStats = HashMap<char, i16>;

pub fn get_letters_stats(words: &[&str]) -> LettersStats {
    let mut stats = LettersStats::new();

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