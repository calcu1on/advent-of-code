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
    // get all matches of do and their index
    // get all matches of dont and their index
    let mut part2_sum = 0;
    let dont_regex = Regex::new("don't\\(\\)").unwrap();
    let do_regex = Regex::new("do\\(\\)").unwrap();
    let donts: Vec<u64> = dont_regex.find_iter(&file).map(|m| m.start() as u64).collect();
    let dos: Vec<u64> = do_regex.find_iter(&file).map(|m| m.start() as u64).collect();

    for mul in re.captures_iter(&file) {
        let start: usize = mul.get(0).unwrap().start();
        // get the closest number, that is lower than the start value
        let mut closest_dont = 0;
        for i in (0..start).rev() {
            let mut index = i as u64;
            if donts.contains(&index) {
                closest_dont = index;
                break;
            }
        }
        let mut closest_do = 0;
        for i in (0..start).rev() {
            let mut index = i as u64;
            if dos.contains(&index) {
                closest_do = index;
                break;
            }
        }

        if closest_do > closest_dont {
            let num1: &u32 = &mul[1].parse().expect("Conversion failed");
            let num2: &u32 = &mul[2].parse().expect("Conversion failed");
            let total = num1 * num2;
            part2_sum += total;  
        }

        if closest_do == 0 && closest_dont == 0 {
            let num1: &u32 = &mul[1].parse().expect("Conversion failed");
            let num2: &u32 = &mul[2].parse().expect("Conversion failed");
            let total = num1 * num2;
            part2_sum += total;  
        }
    }

    // Final solutions here.
    let solution = Solution {
        day: 3,
        answer_1: part1_sum.to_string(),
        answer_2: part2_sum.to_string(),
    };
    
    solution
}
