use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct FileReader {
    content: String
}

impl FileReader {
    pub fn new(path: &Path) -> Self {
        let path_name = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("Cant open {}: {}", path_name, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("Cant read {}: {}", path_name, why),
            Ok(_) => print!("File loaded \n"),
        }

        Self {
            content: s
        }
    }

    pub fn content(&self) -> &String {
        return &self.content;
    }
}