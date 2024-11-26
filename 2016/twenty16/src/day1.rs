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
pub fn run_day1_p1() -> Solution {
    // Import the file of instructions.
    let directions_file = fs::read_to_string("./sources/day1.txt").expect("Unable to read file");
    let directions = directions_file.split(",");

    let mut current_position = Position {
        x: 0,
        y: 0,
        facing: 'N',
    };

    for directive in directions {
        let directive = directive.trim();
        let split = split_first_char(directive).unwrap();
        let direction = split.0;
        let amount = split.1.parse::<i32>().unwrap();
        current_position = change_direction(&current_position, direction);

        match current_position.facing {
            'N' => current_position.y = current_position.y + amount,
            'S' => current_position.y = current_position.y - amount,
            'E' => current_position.x = current_position.x + amount,
            'W' => current_position.x = current_position.x - amount,
            _ => current_position.x = current_position.x,
        };
    }

    let answer = current_position.x + current_position.y.abs();
    let solution = Solution {
        day: 1,
        part: "a".to_string(),
        answer: answer.to_string(),
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

