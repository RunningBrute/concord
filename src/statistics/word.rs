use std::collections::HashMap;

pub type WordsStats = HashMap<String, i16>;

pub fn get_words_stats(words: &[&str]) -> WordsStats {
    // words.swap_remove(0);
    let mut stats = WordsStats::new();

    for &word in words {
        match stats.get_mut(word) {
            Some(value) => update_word_frequency(&word, value),
            None => add_new_word_to_stats(word, &mut stats),
        }
    }

    stats
}

/// Function update `frequency` parameter by one each time
/// when function `update_word_frequency` is called.
/// 
/// ```
/// use concodr::statistics::word;
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

fn add_new_word_to_stats(word: &str, stats: &mut WordsStats) {
    println!("New word added: {}", word);
    stats.insert(word.to_string(), 1);
}
