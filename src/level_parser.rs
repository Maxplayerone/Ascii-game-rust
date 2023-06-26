use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_level() -> Vec<char>{
    // Open the file in read-only mode
    let file = File::open("src/level_one.sag").unwrap();

    // Create a buffer reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file
    let mut map: Vec<char> = Vec::new();
    for line in reader.lines() {
        // Unwrap the line or handle the error gracefully
        let line: String = line.unwrap();
        for c in line.chars(){
           map.push(c);
        }
        map.push('\n');
    }
    map
}