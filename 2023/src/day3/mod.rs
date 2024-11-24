use crate::utils::read_lines;
use linecount::count_lines;
use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn process() -> u32 {
    let file_path = "./input_sources/day3short.txt";
    let mut total: u32 = 0;
    
    let mut lines_vec: Vec<String> = vec![];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
           lines_vec.push(line); 
        }
    }
    let length: usize = lines_vec.len();

    let mut i = 0;
    while i < length {
        let cur_line = &lines_vec[i];
        println!("{:?}", &lines_vec[i]);
        
        if i < 9 {
            let next_line = &lines_vec[i + 1];
        }
        if i > 0 {
            let prev_line = &lines_vec[i - 1];
            println!("Prev line is {:?}", prev_line);
        }

        i = i + 1;
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
