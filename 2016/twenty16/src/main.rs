use tabled::{Table, Tabled};
pub mod day1;

#[derive(Tabled)]
pub struct Solution {
    day: u64,
    part: String,
    answer: String,
}

fn main() {
    let mut rows = Vec::new();
    let day1_p1 = day1::run_day1_p1();
    rows.push(day1_p1);

    let table = Table::new(rows);

    print!("{}", table);

}
