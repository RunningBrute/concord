use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct FileReader {
    content: String,
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

        Self { content: s }
    }

    pub fn content(&self) -> &String {
        return &self.content;
    }
}

#[cfg(test)]
mod tests {

use super::*;

    fn create_example_file(path: &Path, content: &str){
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    #[test]
    fn empty_file(){
        static EXPECTED_CONTENT: &str = "";
        let path: &Path = Path::new("example_file.txt");
        create_example_file(path, EXPECTED_CONTENT);

        let file_reader: FileReader = FileReader::new(path);

        assert_eq!(file_reader.content(), EXPECTED_CONTENT);
    }

    #[test]
    fn file_with_content(){
        static EXPECTED_CONTENT: &str = "hello world a ab abc";
        let path = Path::new("example_file.txt");
        create_example_file(path, EXPECTED_CONTENT);

        let file_reader: FileReader = FileReader::new(path);

        assert_eq!(file_reader.content(), EXPECTED_CONTENT);
    }
}
