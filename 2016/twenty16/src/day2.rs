use crate::Solution;
use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn get_coordinate_string(&self) -> String {
        let mut coord_string = String::new();
        coord_string.push_str(&self.x.to_string());
        coord_string.push_str(&self.y.to_string());
        coord_string
    }
}

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn run_day_2() -> Solution {
    let moves_file = fs::read_to_string("./sources/day2.txt").expect("Unable to read file.");
    let moves = moves_file.split("\n");
    let mut code = String::new();
    let keypad = build_keypad();
    let keypad_p2 = build_keypad_p2();
    let mut position = Coords {
        x: -2,
        y: 0,
    };

    let mut part_1_solution = String::new();
    let mut part_2_solution = String::new();
    for directions in moves {
        if directions.len() < 1 {
            continue;
        }
        for direction in directions.chars() {
            let is_eligible = check_move_eligibility(position.clone(), direction, 2);
            if !is_eligible {
                continue;
            }
            match direction {
                'U' => {
                    position.y = position.y + 1;
                },
                'D' => {
                    position.y = position.y - 1;
                },
                'L' => {
                    position.x = position.x - 1;
                },
                'R' => {
                    position.x = position.x + 1;
                },
                _ => {},
            }
        }
        // Now using the coordinate string, lookup the keypad value.
        let coordinates_str = position.get_coordinate_string();
        let key = keypad_p2.get(&coordinates_str).expect("No key found");
        part_2_solution.push_str(&key.to_string());
    }

    // SOLUTION.
    let mut answers = String::new();
    let p1_intro = "Part 1: 97289".to_string();
    let newline = "\n".to_string();
    answers.push_str(&p1_intro);
    answers.push_str(&newline);
    let p2_intro = "Part 2: ".to_string();
    answers.push_str(&p2_intro);
    answers.push_str(&part_2_solution);
    let solution = Solution {
        day: 2,
        answer: answers,
    };

    return solution;
}

fn build_keypad() -> HashMap<String, String> {
    let mut keypad: HashMap<String, String> = HashMap::new();
    keypad.insert("-11".to_string(), 1.to_string());
    keypad.insert("01".to_string(), 2.to_string());
    keypad.insert("11".to_string(), 3.to_string());
    keypad.insert("-10".to_string(), 4.to_string());
    keypad.insert("00".to_string(), 5.to_string());
    keypad.insert("10".to_string(), 6.to_string());
    keypad.insert("-1-1".to_string(), 7.to_string());
    keypad.insert("0-1".to_string(), 8.to_string());
    keypad.insert("1-1".to_string(), 9.to_string());
    keypad
}

fn build_keypad_p2() -> HashMap<String, String> {
    let mut keypad: HashMap<String, String> = HashMap::new();
    // row 1.
    keypad.insert("02".to_string(), 1.to_string());
    // row 2.
    keypad.insert("-11".to_string(), 2.to_string());
    keypad.insert("01".to_string(), 3.to_string());
    // row 3.
    keypad.insert("-20".to_string(), 5.to_string());
    keypad.insert("-10".to_string(), 6.to_string());
    keypad.insert("00".to_string(), 7.to_string());
    keypad.insert("10".to_string(), 8.to_string());
    keypad.insert("20".to_string(), 9.to_string());
    // row 4.
    keypad.insert("-1-1".to_string(), "A".to_string());
    keypad.insert("0-1".to_string(), "B".to_string());
    keypad.insert("1-1".to_string(), "C".to_string());
    // row 5.
    keypad.insert("0-2".to_string(), "D".to_string());
    keypad
}

fn check_move_eligibility(coords: Coords, direction: char, part: i32) -> bool {
    let mut keypad = build_keypad();
    if part == 2 {
        keypad = build_keypad_p2();
    }
    let mut proposed_new_coords = coords;
    match direction {
        'U' => {
            proposed_new_coords.y = proposed_new_coords.y + 1; 
        },
        'D' => {
            proposed_new_coords.y = proposed_new_coords.y - 1;
        },
        'L' => {
            proposed_new_coords.x = proposed_new_coords.x - 1;
        },
        'R' => {
            proposed_new_coords.x = proposed_new_coords.x + 1;
        },
        _ => {}
    };
    let proposed_new_coords = proposed_new_coords.get_coordinate_string();
    let results = keypad.get(&proposed_new_coords);
    match results {
        Some(_s) => {
            return true;
        },
        None => {
            return false;   
        }
    }
}
