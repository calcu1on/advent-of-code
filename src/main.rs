#![allow(warnings)]
mod day1;
mod day2;
mod utils;

fn main() {

    let day1_results = day1::process();
    let day2_results = day2::process();

    println!("The Answer to Day 1 is: {:?}", day1_results);
    println!("The Answer to Day 2 is: {:?}", day2_results);

}

