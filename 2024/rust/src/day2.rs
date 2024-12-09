use crate::Solution;
use std::fs;
use std::process;

pub fn run_day_2() -> Solution {
    // Fetch the file.
    let reports_file = fs::read_to_string("./inputs/day2.txt").expect("unable to read file");
    // Init the safe reports count.
    let mut safe_reports_p1 = 0;
    let mut safe_reports_p2 = 0;
    // iterate over each line to check if it matches the conditions.
    for line in reports_file.lines() {
        let line_string = line.to_string();
        // create a vector with each character for iteration.
        let report_numbers: Vec<u32> = line_string.split_whitespace().map(|s| s.parse().expect("Conversion Failed")).collect();
        // Now we have report_nums populated with the numbers to check.
        let valid_report = validate(&report_numbers);
        if valid_report {
            safe_reports_p1 += 1;
            safe_reports_p2 += 1;
            continue;
        }
        // remove each item one at a time and re-validate.
        let mut dampened = false;
        for i in 0..report_numbers.len() {
            let mut report_copy = report_numbers.clone();
            report_copy.remove(i);
            if validate(&report_copy) {
               dampened = true; 
               break;
            }
            
        }
        if dampened {
            safe_reports_p2 += 1;
        }
    }

    // Final solutions go here.
    let solution = Solution {
        day: 2,
        answer_1: safe_reports_p1.to_string(),
        answer_2: safe_reports_p2.to_string(),
    };

    solution
}

fn validate(numbers: &Vec<u32>) -> bool {
    let increasing = is_increasing(&numbers);
    let decreasing = is_decreasing(&numbers);
    let valid_diffs = check_diffs(&numbers);
    let mut valid = false;
    
    if increasing || decreasing {
        if valid_diffs {
            valid = true;
        }
    }

    valid
}

fn is_increasing(report_nums: &Vec<u32>) -> bool {
    let mut increasing = false;
    for i in 0..report_nums.len() {
        // If we are on the last iteration, continue.
        if i == report_nums.len() - 1 {
            continue;
        }
        let current_num = report_nums[i as usize];
        let next_num = report_nums[(i as i32 + 1) as usize];
        if next_num <= current_num {
            increasing = false;
            break;
        } else {
            increasing = true;
        }
    }
    increasing
}

fn is_decreasing(report_nums: &Vec<u32>) -> bool {
    let mut decreasing = false;
    for i in 0..report_nums.len() {
        // If we are on the last iteration, continue.
        if i == report_nums.len() - 1 {
            continue;
        }
        let current_num = report_nums[i as usize];
        let next_num = report_nums[(i as i32 + 1) as usize];
        if next_num >= current_num {
            decreasing = false;
            break;
        } else {
            decreasing = true;
        }
    }
    decreasing
}

fn check_diffs(report_numbers: &Vec<u32>) -> bool {
    let mut valid_diffs = true;
    let _last_iteration = report_numbers.len();
    for i in 0..report_numbers.len() {
        let current_number = report_numbers[i as usize];
        match i {
            0 => {
               let next_number: u32 = report_numbers[(i as u32 + 1) as usize];
               let diff_1: u32 = diff(current_number, next_number);
               if !is_valid_diff(diff_1) { 
                   valid_diffs = false;
               }
            },
            last_iteration => {
               let prior_number: u32 = report_numbers[(i as u32 - 1) as usize];
               let diff_1: u32 = diff(current_number, prior_number);
               if !is_valid_diff(diff_1) { 
                   valid_diffs = false;
               }
            },
            _ => {
               let next_number: u32 = report_numbers[(i as u32 + 1) as usize];
               let prior_number: u32 = report_numbers[(i as u32 - 1) as usize];
               let diff_1: u32 = diff(current_number, next_number);
               let diff_2: u32 = diff(current_number, prior_number);
               if !is_valid_diff(diff_1) || !is_valid_diff(diff_2) { 
                   valid_diffs = false;
               }
            }
        }

        if !valid_diffs {
            return false;
        }
    }
    true
}

fn diff(a: u32, b: u32) -> u32 {
    if a < b {
        return b - a;
    }

    return a - b;
}

fn is_valid_diff(diff: u32) -> bool {
    let mut valid = true;
    if diff > 3  || diff < 1 {
        valid = false;
    }
    valid
}
