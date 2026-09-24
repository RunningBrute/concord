use std::collections::HashMap;

pub type WordsStats = HashMap<String, i16>;

/// Function return map with frequency of word in input vector of string.
/// 
/// ```
/// use concord::statistics::word;
/// 
/// let input: Vec<&str> = vec!["hello", "world", "hello"];
/// let result = word::get_words_stats(&input);
/// 
/// assert_eq!(*result.get("hello").unwrap(), 2);
/// assert_eq!(*result.get("world").unwrap(), 1);
/// ```
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

fn update_word_frequency(word: &str, frequency: &mut i16) {
    *frequency = *frequency + 1;
    println!("Word already exist: {}, {}", word, *frequency);
}

fn add_new_word_to_stats(word: &str, stats: &mut WordsStats) {
    println!("New word added: {}", word);
    stats.insert(word.to_string(), 1);
}
