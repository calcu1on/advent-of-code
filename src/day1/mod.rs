use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn process() -> u32 {
    process_day_1_part_2()
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct NumWord {
    word: String,
    value: u32,
    location: u32
}

#[allow(unused_mut)]
#[allow(unused_variables)]
fn process_day_1_part_2() -> u32 {
    let file_path = "./input_sources/day1short.txt";
    let mut to_sum: Vec<u32> = vec![];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let mut num_string: String = String::from("");
            let mut first_number: bool = false;
            let mut second_number: bool = false;
            
            println!("\n{}", line);

            // Check if first char is numeric.
            let first_ch = line.chars().next().unwrap();
            if first_ch.is_numeric() {
               num_string.push(first_ch);
               first_number = true;
            }
            
            // Check if last char is numeric.
            let last_ch = line.chars().last().unwrap();
            if last_ch.is_numeric() {
                num_string.push(last_ch);
                second_number = true;
            }

            // If we have both of these filled, combine and continue.
            if first_number && second_number {
                let to_int: u32 = num_string.trim().parse().unwrap();
                to_sum.push(to_int);
                println!("Solution: {}", num_string);
                continue;
            }

            // Need to search for strings within the line and return their index.
            // Build a struct (  )
            let num_words = HashMap::from([
                ("one".to_string(), 1),
                ("two".to_string(), 2),
                ("three".to_string(), 3),
                ("four".to_string(), 4),
                ("five".to_string(), 5),
                ("six".to_string(), 6),
                ("seven".to_string(), 7),
                ("eight".to_string(), 8),
                ("nine".to_string(), 9)
            ]);
                      
            let mut words_info: HashMap<u32, u32> = HashMap::new();
            for (word, val) in num_words {
                let found_word: Option<usize> = line.find(&word);
                match found_word {
                    None => {
                    },
                    Some(index) => {
                        words_info.insert(val, index as u32);
                        println!("Found word: {} at location {}", word, index);
                    }
                }
            }
            let mut count_vec: Vec<_> = words_info.iter().collect();
            count_vec.sort_by(|a, b| a.1.cmp(b.1));

            // If we have a first char, get the numeric value for the last item
            let word_num_count = count_vec.len();

            if first_number {
                if word_num_count > 0 {
                    let last_word_num = count_vec.clone().into_iter().last();
                    match last_word_num {
                        None => {
                        },
                        Some(tupval) => {
                        }
                    }
                } 
            }

            if second_number {
                if word_num_count > 0 {
                    let first_word_num = count_vec.clone().into_iter().nth(0);
                    match first_word_num {
                        None => {
                        },
                        Some(tupval) => {
                            num_string = tupval.0.to_string() + &num_string;
                        }
                    }
                } 
            }

            if !first_number && !second_number {
                
        
            }


            // If we have a last char, get the numeric value for the first item
            // If we have no chars, 
            //   see if we have one item, if we do, repeat it twice
            //   see if we have two items, if we do, add them sequentially
            //   see if we have > 2 items, if we do, add the first and the last item.
            println!("First Char: {} - Last Char {}", first_ch, last_ch);
            println!("Solution: {}", num_string);
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

#[allow(dead_code)]
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
