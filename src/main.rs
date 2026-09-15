mod statistics;

use std::collections::HashMap;
use statistics::Statistics;

fn main() {
    let words: Vec<String> = std::env::args().collect();

    print_words(&words);

    // let _statistics: HashMap<String, i16> = get_words_stats(words);
    get_all_stats(words);
}

fn print_words(words: &Vec<String>) {
    println!("Words from cli:");

    for word in words {
        println!("  • {}", word);
    }
}

fn get_all_stats(words: Vec<String>) -> Statistics {
    let result: Statistics = Statistics::new();
    return result;
}

fn get_words_stats(words: Vec<String>) -> HashMap<String, i16> {
    let mut stats: HashMap<String, i16> = HashMap::new();

    // words.swap_remove(0);

    for word in words {
        match stats.get_mut(&word) {
            Some(value) => update_word_frequency(&word, value),
            None => add_new_word_to_stats(word, &mut stats),
        }
    }

    return stats;
}

fn update_word_frequency(word: &String, frequency: &mut i16) {
    *frequency = *frequency + 1;
    println!("Word already exist: {}, {}", word, *frequency);
}

fn add_new_word_to_stats(word: String, stats: &mut HashMap<String, i16>) {
    println!("New word added: {}", word);
    stats.insert(word, 1);
}
