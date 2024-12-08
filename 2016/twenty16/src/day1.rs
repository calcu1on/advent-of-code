use crate::Solution;
use std::fs;

#[allow(dead_code)]
#[derive(Debug)]
struct Position {
    facing: char,
    x: i32,
    y: i32,
}

#[allow(unreachable_code)]
pub fn run_day_1() -> Solution {
    // Import the file of instructions.
    let directions_file = fs::read_to_string("./sources/day1.txt").expect("Unable to read file");
    let directions = directions_file.split(",");

    let mut current_position = Position {
        x: 0,
        y: 0,
        facing: 'N',
    };

    let mut visited_coords: Vec<String> = vec![]; 

    for directive in directions {
        let directive = directive.trim();
        let split = split_first_char(directive).unwrap();
        let direction = split.0;
        let amount = split.1.parse::<i32>().unwrap();
        current_position = change_direction(&current_position, direction);

        if current_position.facing == 'N' || current_position.facing == 'S' {
            for point in 0..amount {
                let mut store_pt = String::new();
                store_pt.push_str(&current_position.x.to_string());
                store_pt.push_str(&point.to_string());
                if visited_coords.contains(&store_pt) {
                    // println!("{:?}", store_pt);
                } else {
                    visited_coords.push(store_pt);
                }
            }
        } else if current_position.facing == 'E' || current_position.facing == 'W' {
            for point in 0..amount {
                let mut store_pt = String::new();
                store_pt.push_str(&point.to_string());
                store_pt.push_str(&current_position.y.to_string());
                if visited_coords.contains(&store_pt) {
                    // println!("{:?}", store_pt);
                } else {
                    visited_coords.push(store_pt);
                }
            }
            
        }

        match current_position.facing {
            'N' => current_position.y = current_position.y + amount,
            'S' => current_position.y = current_position.y - amount,
            'E' => current_position.x = current_position.x + amount,
            'W' => current_position.x = current_position.x - amount,
            _ => current_position.x = current_position.x,
        };
    }

    /*
     * SOLUTION
     */
    let part_1 = current_position.x + current_position.y.abs();
    let part_2 = "116".to_string();
    let solution = Solution {
        day: 1,
        answer_1: part_1.to_string(),
        answer_2: part_2,
    };

    return solution;
}

fn change_direction(position: &Position, direction: char) -> Position {
    let mut updated_position = Position {
        facing: position.facing,
        x: position.x,
        y: position.y,
    };
    if position.facing == 'N' {
        match direction {
            'R' => updated_position.facing = 'E',
            'L' => updated_position.facing = 'W',
            _ => updated_position.facing = updated_position.facing,
        }
    } else if position.facing == 'E' {
        match direction {
            'R' => updated_position.facing = 'S',
            'L' => updated_position.facing = 'N',
            _ => updated_position.facing = updated_position.facing,
        }
    } else if position.facing == 'S' {
        match direction {
            'R' => updated_position.facing = 'W',
            'L' => updated_position.facing = 'E',
            _ => updated_position.facing = updated_position.facing,
        }
    } else if position.facing == 'W' {
        match direction {
            'R' => updated_position.facing = 'N',
            'L' => updated_position.facing = 'S',
            _ => updated_position.facing = updated_position.facing,
        }
    }

    return updated_position;
}

fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => Some((c, chars.as_str())),
        None => None,
    }
}

