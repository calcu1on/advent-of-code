use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn process() -> u32 {
    process_day_1_part_2()
}

#[allow(dead_code)]
fn process_day_1() -> u32 {
    let file_path = "/Users/dan/Coding/rust/adventofcode/input_sources/day1.txt";
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

#[allow(unused_mut)]
#[allow(unused_variables)]
fn process_day_1_part_2() -> u32 {
    let file_path = "/Users/dan/Coding/rust/adventofcode/input_sources/day1short.txt";
    let mut to_sum: Vec<u32> = vec![];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let mut num_string: String = String::from("");
            let mut first_number: bool = false;
            let mut second_number: bool = false;
            println!("{}", line);
            // Check if first or last char is numeric
            let first_ch = line.chars().next().unwrap();
            if first_ch.is_numeric() {
               num_string.push(first_ch);
               first_number = true;
            }
            let last_ch = line.chars().last().unwrap();
            if last_ch.is_numeric() {
                num_string.push(last_ch);
                second_number = true;
            }

            // If we have both of these filled, combine and continue.
            if first_number && second_number {
                let to_int: u32 = num_string.trim().parse().unwrap();
                to_sum.push(to_int);
                println!("Slots Full!! {}{}\n", first_ch, last_ch);

                continue;
            }
            // Need to search for strings within the line and return their index.
            // Build a struct (  )
            let num_words: Vec<String> = vec![
                String::from("one"),
                String::from("two"),
                String::from("three"),
                String::from("four"),
                String::from("five"),
                String::from("six"),
                String::from("seven"),
                String::from("eight"),
                String::from("nine"),
            ];
            
            for word in num_words {
                let found_word: Option<usize> = line.find(&word);
                match found_word {
                    None => {
                    },
                    Some(index) => {

                        println!("Found word: {} at location {}", word, index)
                    }
                }
            }

            println!("First Char: {} - Last Char {}\n", first_ch, last_ch);
            // Create an index of valid words +location
            // Valid numbers plus location
            // Could do quick check to see if last char is numeric, if so we know this is the last
            //  number for the return string.
            // Will need a way to convert strings to digits (one -> 1)
        }
    }

    println!("\nNumbers to Sum: {:?}", to_sum);
    420 as u32
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
