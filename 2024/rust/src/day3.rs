use crate::Solution;
use std::fs;
use std::process;
use regex::Regex;

pub fn run_day_3() -> Solution {
    
    let file = fs::read_to_string("./inputs/day3.txt").expect("Unable to parse file");
    let re = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
    let mut part1_sum = 0;
    for mul in re.captures_iter(&file) {
        // Multiply the second and the third matches.
        let num1: &u32 = &mul[1].parse().expect("Conversion failed");
        let num2: &u32 = &mul[2].parse().expect("Conversion failed");
        let total = num1 * num2;
        part1_sum += total;
    }

    
    let solution = Solution {
        day: 3,
        answer_1: part1_sum.to_string(),
        answer_2: "Not sure".to_string(),
    };
    
    solution
}
