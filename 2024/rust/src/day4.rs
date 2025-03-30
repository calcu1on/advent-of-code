use crate::Solution;
use std::fs;
use std::process;

pub fn run_day_4() -> Solution {
    let file = fs::read_to_string("./inputs/day4test.txt").expect("Unable to parse file");
    let search_map = build_search_map(&file);
    // @here you are Dan
    // this is now creating a vector which you can iterator over
    // will need to work out logic to go through each letter
    // and detect if it is part of a match or now
    // somehow keeping track of past matches
    // need to do in all directions and backwards
    // this is...complicated.
    dbg!(search_map[0][2]);

    // Return the solution
    let solution = Solution {
        day: 4,
        answer_1: "test".to_string(),
        answer_2: "another".to_string(),
    };
    solution
}

fn build_search_map(row: &str) -> Vec<Vec<char>> {
    let mut search_map: Vec<Vec<char>> = vec!();
    for line in row.lines() {
        let char_vec: Vec<char> = line_to_chars(&line);
        search_map.push(char_vec.clone()); 
    }
    search_map
}


fn line_to_chars(row_line: &str) -> Vec<char> {
    row_line.chars().collect()
}

