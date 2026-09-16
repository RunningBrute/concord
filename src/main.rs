mod statistics;
mod file_reader;

use statistics::Statistics;
use file_reader::FileReader;
use std::collections::HashMap;
use std::path::Path;
use std::ptr::read;

fn main() {
    //let words: Vec<String> = std::env::args().collect();

    //print_words(&words);

    // let _statistics: HashMap<String, i16> = get_words_stats(words);
    //let _result: Statistics = get_all_stats(words);

    let file_path = Path::new("data/input.txt");
    let reader: FileReader = FileReader::new(&file_path);
    print!("{}", reader.content());
    //let _result: Statistics = get_all_stats(reader.content());
}

fn print_words(words: &Vec<String>) {
    println!("Words from cli:");

    for word in words {
        println!("  • {}", word);
    }
}

fn get_all_stats(words: Vec<String>) -> Statistics {
    let mut result: Statistics = Statistics::new();

    get_words_stats(words.clone(), &mut result.words_mut());
    get_letters_stats(words.clone(), &mut result.letters_mut());

    return result;
}

fn get_words_stats(words: Vec<String>, mut stats: &mut HashMap<String, i16>) {
    // words.swap_remove(0);

    for word in words {
        match stats.get_mut(&word) {
            Some(value) => update_word_frequency(&word, value),
            None => add_new_word_to_stats(word, &mut stats),
        }
    }
}

fn get_letters_stats(words: Vec<String>, stats: &mut HashMap<char, i16>) {
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

fn update_word_frequency(word: &String, frequency: &mut i16) {
    *frequency = *frequency + 1;
    println!("Word already exist: {}, {}", word, *frequency);
}

fn add_new_word_to_stats(word: String, stats: &mut HashMap<String, i16>) {
    println!("New word added: {}", word);
    stats.insert(word, 1);
}
