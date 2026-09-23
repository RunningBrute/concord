mod file_reader;
mod statistics;
mod stats_printer;

use file_reader::FileReader;
use statistics::Statistics;
use std::path::Path;
use std::str::SplitWhitespace;

use crate::stats_printer::*;

fn main() {
    //let words: Vec<String> = std::env::args().collect();
    //print_words(&words);
    //let _statistics: BTreeMap<String, i16> = get_words_stats(words);
    //let result: Statistics = get_all_stats(words);
    //print_words_stats(&result.words());
    //print_letters_stats(&result.letters());

    let file_path: &Path = Path::new("data/input.txt");
    let reader: FileReader = FileReader::new(&file_path).unwrap();
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

    let result: Statistics = Statistics::from(&input);

    print_words_stats(&result.words());
    print_letters_stats(&result.letters());
}
