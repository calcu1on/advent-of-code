use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn process() -> u32 {
    process_day_1_part_2()
}

fn process_day_1() -> u32 {
    let file_path = "./input_sources/day1.txt";
    let mut to_sum: Vec<u32> = vec![];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let numbers = extract_numbers_from_string(line.clone());
            to_sum.push(numbers);
        }
    }
    let sum: u32 = to_sum.iter().sum();
    sum
}

fn process_day_1_part_2() -> u32 {
    let file_path = "./input_sources/day1.txt";
    let mut to_sum: Vec<u32> = vec![];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let line_result: u32 = process_line(line);
            to_sum.push(line_result);
        }
    }

//    //println!("\nNumbers to Sum: {:?}", to_sum);
    let sum: u32 = to_sum.iter().sum();
    sum
}

fn process_line(input_line: String) -> u32 {
    //println!("{}", input_line);
    let mut num_string: String = String::new();
    let mut collector: Vec<(u32, u32)> = vec![];
    let mut r_collector: Vec<(u32, u32)> = vec![];
    let targets = HashMap::from([
                                  ("zero".to_string(), 0),
                                  ("0".to_string(), 0),
                                  ("one".to_string(), 1),
                                  ("1".to_string(), 1),
                                  ("two".to_string(), 2),
                                  ("2".to_string(), 2),
                                  ("three".to_string(), 3),
                                  ("3".to_string(), 3),
                                  ("four".to_string(), 4),
                                  ("4".to_string(), 4),
                                  ("five".to_string(), 5),
                                  ("5".to_string(), 5),
                                  ("six".to_string(), 6),
                                  ("6".to_string(), 6),
                                  ("seven".to_string(), 7),
                                  ("7".to_string(), 7),
                                  ("eight".to_string(), 8),
                                  ("8".to_string(), 8),
                                  ("nine".to_string(), 9), 
                                  ("9".to_string(), 9)
    ]);

    for (target, value) in targets {
        let found_target: Option<usize> = input_line.find(&target);
        let r_found_target: Option<usize> = input_line.rfind(&target);

        match found_target {
            None => {
            },
            Some(index) => {
                collector.push((value, index as u32));
                //println!("Found {} at {}", value, index);
            }
        }

        match r_found_target {
            None => {
            },
            Some(index) => {
                collector.push((value, index as u32));
                //println!("Rfound {} at {}", value, index);
            }
        }


    }

    collector.sort_by_key(|k| k.1);
    let mut first_number: String = String::new();    
    let mut last_number: String = String::new();
    // Wacky override 
    if input_line.chars().last().unwrap().is_numeric() {
        last_number = input_line.clone().chars().last().unwrap().to_string(); 
    } else {
        last_number = collector.last().unwrap().0.to_string();
    }
    // Wacky override 
    if input_line.chars().nth(0).unwrap().is_numeric() {
        first_number = input_line.clone().chars().nth(0).unwrap().to_string(); 
    } else {
        first_number = collector.first().unwrap().0.to_string();
    }
    //println!("{:?}", String::from(first_number.to_owned() + &last_number).trim().parse::<u32>().unwrap());

    String::from(first_number.to_owned() + &last_number).trim().parse::<u32>().unwrap()
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_numbers_from_string(input: String) -> u32 {
    let mut nums_in_string: Vec<char> = vec![];
    for char in input.chars() {
        if char.is_numeric() {
            nums_in_string.push(char);
        }
    }

    let char_count = nums_in_string.len();
    let number: u32;
    if char_count == 1 {
        // add chars to string.
        let num_string = nums_in_string[0].clone().to_string() + &nums_in_string[0].clone().to_string();
        // convert to u32.
        number = num_string.trim().parse().unwrap();
    } else {
        let first_item = nums_in_string[0].clone();
        let last_item = nums_in_string.last().copied();
        let num_string = first_item.to_string() + &last_item.expect("should be two chars.").to_string();
        number = num_string.trim().parse().unwrap()
    }
    number
}

#[test]
fn test_extract_numbers_from_string() {
    let expected: u32 = 74;
    let test_value = String::from("742fiveeightnvjjpx4eight");
    let actual = extract_numbers_from_string(test_value);
    assert_eq!(expected, actual);
}

#[test]
fn test_process() {
    let expected: u32 = 54927;
    let actual: u32 = process_day_1(); 
    assert_eq!(expected, actual);
}
