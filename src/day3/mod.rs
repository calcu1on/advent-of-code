use crate::utils::read_lines;
use linecount::count_lines;
use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn process() -> u32 {
    let file_path = "./input_sources/day3short.txt";
    let mut total: u32 = 0;
    let i = 2;
    if let Ok(mut lines) = read_lines(file_path) {
        let mut cur_line = lines.nth(i).expect("no current line");
        let length: usize = count_lines(File::open(file_path).unwrap()).unwrap();
        println!("{:?}", length);
        if i > 0 {
            let mut next_line = lines.nth(i + 1).expect("no next line found.");
            let mut prev_line = lines.nth(i - 1).expect("no previous line found.");
            println!("{:?}", prev_line);
            println!("{:?}", next_line);
        }
        
    }
    total
}


fn handle_line(input_line: String) -> String {
    // Get previous line.
    // Get next line.
    // On current line, iterate over each char
    // build digits and their locations.
    // For each digit
    //  go to prior line, check if symbol in (place, place +1, place -1)
    //  do the same for the next line.
    // If a symbol is found, return true for the number
    // Sum numbers

   // println!("{:?}", input_line);
    String::from("Hello")
}
