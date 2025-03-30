use crate::Solution;
use std::fs;
use std::process;

struct Direction {
    x: i32,
    y: i32,
}

pub fn run_day_4() -> Solution {
    let file = fs::read_to_string("./inputs/day4test.txt").expect("Unable to parse file");
    let search_map = build_search_map(&file);
    let directions: Vec<(i32, i32)> = vec![
        (0,1), // N
        (0,-1), // S
        (1,0), // E
        (-1,0), // W
        (1,1), // NE
        (-1,1), // NW
        (1,-1), // SE
        (-1,-1), // SW
    ];

    // idea 3.0
    // iterate over each letter in the word search
    // for each letter, build all possible words around it
    // we know the character length is 4 (XMAS, SAMX)
    // so if we have a given point, i.e. (3,4)

    dbg!(directions[0].0);

    // Return the solution
    let solution = Solution {
        day: 4,
        answer_1: "test".to_string(),
        answer_2: "another".to_string(),
    };
    solution
}

// Builds a searchable map.
fn build_search_map(row: &str) -> Vec<Vec<char>> {
    let mut search_map: Vec<Vec<char>> = vec!();
    for line in row.lines() {
        let char_vec: Vec<char> = line_to_chars(&line);
        search_map.push(char_vec.clone()); 
    }
    search_map
}

// converts a line into a vectory of chars
fn line_to_chars(row_line: &str) -> Vec<char> {
    row_line.chars().collect()
}
