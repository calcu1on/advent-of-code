#![allow(warnings)]
#![allow(unused)]

use tabled::{Table, Tabled};
pub mod day1;
pub mod day2;
pub mod day3;

#[derive(Tabled)]
pub struct Solution {
    day: u64,
    answer_1: String,
    answer_2: String,
}

impl Solution {
    pub fn build_answer(&self) -> String {
        let mut answer = String::new();
        let part_1_intro = "Part 1: ".to_string();
        let part_2_intro = "Part 2: ".to_string();
        let newline = "\n".to_string();
        answer.push_str(&part_1_intro);
        answer.push_str(&self.answer_1);
        answer.push_str(&newline);
        answer.push_str(&part_2_intro);
        answer.push_str(&self.answer_2);
        answer
    }
}

fn main() {
    let mut rows = Vec::new();
    let day1 = day1::run_day_1();
    let day2 = day2::run_day_2();
    let day3 = day3::run_day_3();
    rows.push(day1);
    rows.push(day2);
    rows.push(day3);

    let table = Table::new(rows);

    print!("{}", table);

}
