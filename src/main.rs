mod file_reader;
mod statistics;
mod stats_printer;

use file_reader::FileReader;
use statistics::Statistics;
use std::collections::BTreeMap;
use std::path::Path;
use std::str::SplitWhitespace;

use crate::stats_printer::{*};

fn main() {
    //let words: Vec<String> = std::env::args().collect();
    //print_words(&words);
    //let _statistics: BTreeMap<String, i16> = get_words_stats(words);
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

fn get_all_stats(words: &[&str]) -> Statistics {
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
