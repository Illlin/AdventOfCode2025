use crate::common_utils::read_file;

mod common_utils;
mod day1;
mod day2;

fn main() {
    let d1input = read_file(String::from("day1"));
    day1::run(d1input);
    let d2input = read_file(String::from("day2"));
    day2::run(d2input);
}
