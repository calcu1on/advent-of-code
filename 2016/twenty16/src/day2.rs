use crate::Solution;
use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
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
    let mut keypad = build_keypad();
    let mut position = Coords {
        x: 0,
        y: 0,
    };

    let mut part_1_solution = String::new();
    for directions in moves {
        if directions.len() < 1 {
            continue;
        }
        for direction in directions.chars() {
            // Need to move around the keypad to determine where we are.
            match direction {
                'U' => {
                   if position.y == 0 || position.y == -1 {
                        position.y = position.y + 1;
                    } 
                },
                'D' => {
                   if position.y == 0 || position.y == 1 {
                        position.y = position.y - 1;
                    }
                },
                'L' => {
                   if position.x == 0 || position.x == 1 {
                        position.x = position.x - 1;
                    }
                },
                'R' => {
                    if position.x == 0 || position.x == -1 {
                        position.x = position.x + 1;
                    }
                },
                _ => {},
            }
        }
        // Now using the coordinate string, lookup the keypad value.
        let coordinates_str = position.get_coordinate_string();
        let key = keypad.get(&coordinates_str).expect("No key found");
        part_1_solution.push_str(&key.to_string());
    }

    // SOLUTION.
    let mut answers = String::new();
    let p1_intro = "Part 1: ".to_string();
    let newline = "\n".to_string();
    answers.push_str(&p1_intro);
    answers.push_str(&part_1_solution);
    answers.push_str(&newline);
    let solution = Solution {
        day: 2,
        answer: answers,
    };

    return solution;
}

fn build_keypad() -> HashMap<String, i32> {
    let mut keypad: HashMap<String, i32> = HashMap::new();
    keypad.insert("-11".to_string(), 1);
    keypad.insert("01".to_string(), 2);
    keypad.insert("11".to_string(), 3);
    keypad.insert("-10".to_string(), 4);
    keypad.insert("00".to_string(), 5);
    keypad.insert("10".to_string(), 6);
    keypad.insert("-1-1".to_string(), 7);
    keypad.insert("0-1".to_string(), 8);
    keypad.insert("1-1".to_string(), 9);
    keypad
}
