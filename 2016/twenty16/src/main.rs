use tabled::{Table, Tabled};
pub mod day1;
pub mod day2;

#[derive(Tabled)]
pub struct Solution {
    day: u64,
    answer: String,
}

fn main() {
    let mut rows = Vec::new();
    let day1 = day1::run_day_1();
    let day2 = day2::run_day_2();
    rows.push(day1);
    rows.push(day2);

    let table = Table::new(rows);

    print!("{}", table);

}
