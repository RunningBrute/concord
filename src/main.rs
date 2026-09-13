use std::collections::HashMap;

fn print_words(words: &Vec<String>) {
    println!("Words from cli:");

    for word in words {
        println!("  • {}", word);
    }
}

fn main() {
    let words: Vec<String> = std::env::args().collect();

    print_words(&words);

    let mut statistics: HashMap<String, i16> = HashMap::new();
    for word in words {
        match statistics.get(&word) {
            Some(value) => println!("Word already exist: {}, {}", word, value),
            None => {
                println!("New word added: {}", word);
                statistics.insert(word, 1);
            }
        }
    }
}
