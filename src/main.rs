mod file_reader;
mod statistics;

use file_reader::FileReader;
use statistics::Statistics;
use std::collections::HashMap;
use std::path::Path;
use std::str::SplitWhitespace;

use crate::statistics::{LettersStats, WordsStats};

fn main() {
    //let words: Vec<String> = std::env::args().collect();
    //print_words(&words);
    //let _statistics: HashMap<String, i16> = get_words_stats(words);
    //let _result: Statistics = get_all_stats(words);

    let file_path: &Path = Path::new("data/input.txt");
    let reader: FileReader = FileReader::new(&file_path);
    let mut iter: SplitWhitespace = reader.content().split_whitespace();
    let mut input: Vec<&str> = Vec::new();

    let mut done: bool = false;
    while !done {
        match iter.next() {
            Some(elem) => input.push(elem),
            None => done = true,
        }
    }

    //print_words(&input);

    let result: Statistics = get_all_stats(&input);

    print_words_stats(&result.words());
    print_letters_stats(&result.letters());
}

fn print_words_stats(words: &WordsStats) {
    let words_count = words.iter().count();
    for word in words {
        let occurences: f32 = (*word.1 as f32 / words_count as f32) * 100.0;
        println!("Word: {}, occurences: {} %", word.0, occurences);
    }
}

fn print_letters_stats(letters: &LettersStats) {
    let letters_count = letters.iter().count();
    for letter in letters {
        let occurences: f32 = (*letter.1 as f32 / letters_count as f32) * 100.0;
        println!("Letter: {}, occurences: {} %", letter.0, occurences);
    }
}

fn print_words(words: &[&str]) {
    println!("All words:");

    for word in words {
        println!("  • {}", word);
    }
}

fn get_all_stats(words: &[&str]) -> Statistics {
    let mut result: Statistics = Statistics::new();

    get_words_stats(words, &mut result.words_mut());
    get_letters_stats(words, &mut result.letters_mut());

    return result;
}

fn get_words_stats(words: &[&str], mut stats: &mut HashMap<String, i16>) {
    // words.swap_remove(0);

    for &word in words {
        match stats.get_mut(word) {
            Some(value) => update_word_frequency(&word, value),
            None => add_new_word_to_stats(word, &mut stats),
        }
    }
}

fn get_letters_stats(words: &[&str], stats: &mut HashMap<char, i16>) {
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

fn add_new_word_to_stats(word: &str, stats: &mut HashMap<String, i16>) {
    println!("New word added: {}", word);
    stats.insert(word.to_string(), 1);
}
