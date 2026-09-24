use crate::statistics::{word::WordsStats, letter::LettersStats};

pub fn print_words_stats(words: &WordsStats) {
    let words_count = words.iter().count();
    for word in words {
        let occurences: f32 = (*word.1 as f32 / words_count as f32) * 100.0;
        println!("Word: {}, occurences: {} %", word.0, occurences);
    }
}

pub fn print_letters_stats(letters: &LettersStats) {
    let letters_count = letters.iter().count();
    for letter in letters {
        let occurences: f32 = (*letter.1 as f32 / letters_count as f32) * 100.0;
        println!("Letter: {}, occurences: {} %", letter.0, occurences);
    }
}

pub fn _print_words(words: &[&str]) {
    println!("All words:");

    for word in words {
        println!("  • {}", word);
    }
}
