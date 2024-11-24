#![allow(warnings)]
mod day1;
mod day2;
mod day3;
mod utils;

fn main() {

    let day1_results = day1::process();
    let day2_results = day2::process();
    let day3_results = day3::process();

    println!("The Answer to Day 1 is: {:?}", day1_results);
    println!("The Answer to Day 2 is: {:?}", day2_results);
    println!("The Answer to Day 3 is: {:?}", day3_results);

}

