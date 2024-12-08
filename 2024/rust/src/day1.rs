use crate::Solution;
use std::fs;

#[allow(dead_code)]
pub fn run_day_1() -> Solution {
    let location_ids = fs::read_to_string("./inputs/day1.txt").expect("Unable to read file");
    let location_lines = location_ids.split("\n");
    // Create location vectors.
    let mut left_locations: Vec<i32> = vec!();
    let mut right_locations: Vec<i32> = vec!();

    for location_line in location_lines {
        if location_line.len() < 1 {
            continue;
        }
        let split_line = location_line.split("   ");
        let collection: Vec<&str> = split_line.collect();
        let left_int: i32 = collection[0].parse().unwrap();
        let right_int: i32 = collection[1].parse().unwrap();
        left_locations.push(left_int);
        right_locations.push(right_int);
    }

    left_locations.sort();
    right_locations.sort();

    let llength = left_locations.len();
    let rlength = right_locations.len();
    if llength != rlength {
        panic!("Vectors has unequal length");
    }


    let mut part_1_distance = 0;
    let mut part_2_distance = 0;

    for i in 0..llength {
        let mut diff = left_locations[i] - right_locations[i];
        diff = diff.abs();
        part_1_distance = part_1_distance + diff;
        // for part 2, we need to check how many times left location appears in right
        // then multiply the number, by the occurances count.
        let lookup = right_locations.iter().filter(|&n| *n == left_locations[i]).count();
        let value = left_locations[i] * lookup as i32;
        part_2_distance = part_2_distance + value;
    }

    let solution = Solution {
        day: 1,
        answer_1: part_1_distance.to_string(),
        answer_2: part_2_distance.to_string(),
    };
    
    return solution;
}
