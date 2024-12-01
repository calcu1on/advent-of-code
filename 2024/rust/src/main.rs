use tabled::{Table, Tabled};
pub mod day1;

#[derive(Tabled)]
pub struct Solution {
    day: u64,
    answer: String,
}

fn main() {
    let mut rows = Vec::new();
    let day1_p1 = day1::run_day_1_part_1();
    rows.push(day1_p1);

    let table = Table::new(rows);

    print!("{}", table);

}
